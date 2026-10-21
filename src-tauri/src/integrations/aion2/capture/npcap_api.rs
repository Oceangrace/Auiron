use super::bpf_program::BpfProgram;
use super::capture_adapter::CaptureAdapter;
use super::pcap_device::PcapDevice;
use super::pcap_header::PcapHeader;
use super::pcap_statistics::PcapStatistics;
use libloading::Library;
use std::ffi::{CStr, c_char, c_void};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::Path;
use windows::Win32::Networking::WinSock::{AF_INET, AF_INET6, SOCKADDR, SOCKADDR_IN, SOCKADDR_IN6};

pub(super) struct NpcapApi {
    _library: Library,
    pub(super) open: unsafe extern "C" fn(*const c_char, i32, i32, i32, *mut c_char) -> *mut c_void,
    pub(super) close: unsafe extern "C" fn(*mut c_void),
    pub(super) nonblock: unsafe extern "C" fn(*mut c_void, i32, *mut c_char) -> i32,
    pub(super) datalink: unsafe extern "C" fn(*mut c_void) -> i32,
    pub(super) compile:
        unsafe extern "C" fn(*mut c_void, *mut BpfProgram, *const c_char, i32, u32) -> i32,
    pub(super) filter: unsafe extern "C" fn(*mut c_void, *mut BpfProgram) -> i32,
    pub(super) freecode: unsafe extern "C" fn(*mut BpfProgram),
    pub(super) next:
        unsafe extern "C" fn(*mut c_void, *mut *const PcapHeader, *mut *const u8) -> i32,
    pub(super) statistics: unsafe extern "C" fn(*mut c_void, *mut PcapStatistics) -> i32,
    pub(super) error: unsafe extern "C" fn(*mut c_void) -> *const c_char,
    find_devices: unsafe extern "C" fn(*mut *mut PcapDevice, *mut c_char) -> i32,
    free_devices: unsafe extern "C" fn(*mut PcapDevice),
}

impl NpcapApi {
    pub(super) fn load(path: &Path) -> Result<Self, String> {
        // SAFETY: absolute configured System32 DLL; dependencies only from its directory/System32.
        let library: Library = unsafe {
            libloading::os::windows::Library::load_with_flags(
                path,
                libloading::os::windows::LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR
                    | libloading::os::windows::LOAD_LIBRARY_SEARCH_SYSTEM32,
            )
        }
        .map_err(|error| format!("Cannot load Npcap: {error}"))?
        .into();

        // SAFETY: the function signatures match the Windows Npcap C API; library outlives pointers.
        unsafe {
            Ok(Self {
                open: *library
                    .get(b"pcap_open_live\0")
                    .map_err(|error| error.to_string())?,
                close: *library
                    .get(b"pcap_close\0")
                    .map_err(|error| error.to_string())?,
                nonblock: *library
                    .get(b"pcap_setnonblock\0")
                    .map_err(|error| error.to_string())?,
                datalink: *library
                    .get(b"pcap_datalink\0")
                    .map_err(|error| error.to_string())?,
                compile: *library
                    .get(b"pcap_compile\0")
                    .map_err(|error| error.to_string())?,
                filter: *library
                    .get(b"pcap_setfilter\0")
                    .map_err(|error| error.to_string())?,
                freecode: *library
                    .get(b"pcap_freecode\0")
                    .map_err(|error| error.to_string())?,
                next: *library
                    .get(b"pcap_next_ex\0")
                    .map_err(|error| error.to_string())?,
                statistics: *library
                    .get(b"pcap_stats\0")
                    .map_err(|error| error.to_string())?,
                error: *library
                    .get(b"pcap_geterr\0")
                    .map_err(|error| error.to_string())?,
                find_devices: *library
                    .get(b"pcap_findalldevs\0")
                    .map_err(|error| error.to_string())?,
                free_devices: *library
                    .get(b"pcap_freealldevs\0")
                    .map_err(|error| error.to_string())?,
                _library: library,
            })
        }
    }

    pub(super) fn adapters(&self) -> Result<Vec<CaptureAdapter>, String> {
        let mut head = std::ptr::null_mut();

        let mut error = [0; 256];

        // SAFETY: Npcap owns the output list; both output pointers are writable.
        let status = unsafe { (self.find_devices)(&mut head, error.as_mut_ptr()) };

        if status != 0 {
            return Err(Self::error_buffer(&error));
        }

        let result = self.copy_adapters(head);

        // SAFETY: successful find_devices allocated this list, including a possible null list.
        unsafe { (self.free_devices)(head) };

        result
    }

    fn copy_adapters(&self, mut device: *mut PcapDevice) -> Result<Vec<CaptureAdapter>, String> {
        let mut result = Vec::new();

        while !device.is_null() {
            if result.len() >= 256 {
                return Err("Npcap adapter count limit".into());
            }

            // SAFETY: list belongs to Npcap and is live until adapters() frees it.
            let record = unsafe { &*device };

            if record.name.is_null() {
                return Err("Npcap adapter name is missing".into());
            }

            // SAFETY: Npcap guarantees a null-terminated adapter name in each list entry.
            let name = unsafe { CStr::from_ptr(record.name) }.to_owned();

            let mut addresses = Vec::new();

            let mut address = record.addresses;

            for _ in 0..256 {
                if address.is_null() {
                    break;
                }

                // SAFETY: address nodes have the same lifetime as the device list.
                let node = unsafe { &*address };

                if let Some(ip) = Self::address(node.address) {
                    addresses.push(ip);
                }

                address = node.next;
            }

            result.push(CaptureAdapter::new(name, addresses, record.flags & 1 != 0));
            device = record.next;
        }

        Ok(result)
    }

    fn address(address: *const SOCKADDR) -> Option<IpAddr> {
        if address.is_null() {
            return None;
        }

        // SAFETY: Npcap provides a sockaddr whose family determines the actual Windows layout.
        unsafe {
            match (*address).sa_family {
                AF_INET => Some(
                    Ipv4Addr::from(
                        (*address.cast::<SOCKADDR_IN>())
                            .sin_addr
                            .S_un
                            .S_addr
                            .to_ne_bytes(),
                    )
                    .into(),
                ),
                AF_INET6 => {
                    Some(Ipv6Addr::from((*address.cast::<SOCKADDR_IN6>()).sin6_addr.u.Byte).into())
                }

                _ => None,
            }
        }
    }

    pub(super) fn error_buffer(bytes: &[c_char]) -> String {
        let end = bytes
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(bytes.len());

        let bytes: Vec<u8> = bytes[..end].iter().map(|byte| *byte as u8).collect();

        String::from_utf8_lossy(&bytes).into_owned()
    }
}
