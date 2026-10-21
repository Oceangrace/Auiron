use windows::Win32::Foundation::{CloseHandle, HANDLE};

pub(super) struct ProcessHandle(HANDLE);

impl ProcessHandle {
    pub(super) fn new(handle: HANDLE) -> Self {
        Self(handle)
    }

    pub(super) fn get(&self) -> HANDLE {
        self.0
    }
}

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        // SAFETY: this guard uniquely owns a valid handle returned by Windows.
        let _ = unsafe { CloseHandle(self.0) };
    }
}
