#[repr(C)]
pub(super) struct PcapDevice {
    pub(super) next: *mut PcapDevice,
    pub(super) name: *mut std::ffi::c_char,
    pub(super) description: *mut std::ffi::c_char,
    pub(super) addresses: *mut super::pcap_address::PcapAddress,
    pub(super) flags: u32,
}
