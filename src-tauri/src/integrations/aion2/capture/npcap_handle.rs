use super::CapturedPacket;
use super::bpf_program::BpfProgram;
use super::npcap_api::NpcapApi;
use super::pcap_statistics::PcapStatistics;
use std::ffi::{CStr, c_void};
use std::ptr::NonNull;
use std::rc::Rc;

pub(super) struct NpcapHandle {
    api: Rc<NpcapApi>,
    handle: NonNull<c_void>,
    link_type: u32,
    dropped: u64,
}

impl NpcapHandle {
    pub(super) fn open(api: Rc<NpcapApi>, adapter: &CStr, filter: &CStr) -> Result<Self, String> {
        let mut error = [0; 256];

        // SAFETY: names/filter are C strings, error buffer has PCAP_ERRBUF_SIZE bytes.
        let raw = unsafe {
            (api.open)(
                adapter.as_ptr(),
                CapturedPacket::MAX_BYTES as i32,
                0,
                100,
                error.as_mut_ptr(),
            )
        };

        let handle = NonNull::new(raw).ok_or_else(|| NpcapApi::error_buffer(&error))?;

        let mut capture = Self {
            api,
            handle,
            link_type: 0,
            dropped: 0,
        };

        capture.configure(filter)?;

        Ok(capture)
    }

    fn configure(&mut self, filter: &CStr) -> Result<(), String> {
        let mut error = [0; 256];

        // SAFETY: live handle, valid buffer and C string, exclusively owned by this thread.
        if unsafe { (self.api.nonblock)(self.handle.as_ptr(), 1, error.as_mut_ptr()) } != 0 {
            return Err(NpcapApi::error_buffer(&error));
        }

        // SAFETY: datalink reads metadata from this open handle.
        let link_type = unsafe { (self.api.datalink)(self.handle.as_ptr()) };

        self.link_type = u32::try_from(link_type).map_err(|_| self.error())?;

        let mut program = BpfProgram {
            count: 0,
            instructions: std::ptr::null_mut(),
        };

        // SAFETY: compile initializes the BPF program; filter is null-terminated.
        let compiled = unsafe {
            (self.api.compile)(
                self.handle.as_ptr(),
                &mut program,
                filter.as_ptr(),
                1,
                u32::MAX,
            )
        };

        if compiled != 0 {
            return Err(self.error());
        }

        // SAFETY: program was successfully compiled; setfilter copies it, then freecode releases it.
        let applied = unsafe { (self.api.filter)(self.handle.as_ptr(), &mut program) };

        unsafe { (self.api.freecode)(&mut program) };

        if applied != 0 {
            return Err(self.error());
        }

        Ok(())
    }

    pub(super) fn next(&mut self) -> Result<Option<CapturedPacket>, String> {
        let mut header = std::ptr::null();

        let mut bytes = std::ptr::null();

        // SAFETY: both output pointer slots are valid; the handle is nonblocking and thread-owned.
        let result = unsafe { (self.api.next)(self.handle.as_ptr(), &mut header, &mut bytes) };

        if result == 0 {
            return Ok(None);
        }

        if result != 1 || header.is_null() || bytes.is_null() {
            return Err(self.error());
        }

        // SAFETY: successful next_ex returns a header valid until the next read on this handle.
        let header = unsafe { &*header };

        if header.captured != header.original
            || header.captured as usize > CapturedPacket::MAX_BYTES
            || header.seconds < 0
            || !(0..1_000_000).contains(&header.microseconds)
        {
            return Err("Truncated or invalid live packet".into());
        }

        // SAFETY: Npcap supplies captured bytes; length was bounded and bytes are copied before next read.
        let data = unsafe { std::slice::from_raw_parts(bytes, header.captured as usize) }.to_vec();

        let timestamp = header.seconds as u64 * 1_000_000 + header.microseconds as u64;

        CapturedPacket::new(timestamp, self.link_type, data).map(Some)
    }

    pub(super) fn lost_packets(&mut self) -> Result<bool, String> {
        let mut stats = PcapStatistics {
            received: 0,
            dropped: 0,
            interface_dropped: 0,
            captured: 0,
        };

        // SAFETY: stats is a writable Windows pcap_stat record for this live handle.
        let result = unsafe { (self.api.statistics)(self.handle.as_ptr(), &mut stats) };

        if result != 0 {
            return Err(self.error());
        }

        let dropped = u64::from(stats.dropped) + u64::from(stats.interface_dropped);

        let changed = dropped != self.dropped;

        self.dropped = dropped;

        Ok(changed)
    }

    fn error(&self) -> String {
        // SAFETY: geterr returns an internal C string valid while the handle remains open.
        let pointer = unsafe { (self.api.error)(self.handle.as_ptr()) };

        if pointer.is_null() {
            return "Npcap operation failed".into();
        }

        unsafe { CStr::from_ptr(pointer) }
            .to_string_lossy()
            .into_owned()
    }
}

impl Drop for NpcapHandle {
    fn drop(&mut self) {
        // SAFETY: unique open handle, retained library, no concurrent reads.
        unsafe { (self.api.close)(self.handle.as_ptr()) };
    }
}
