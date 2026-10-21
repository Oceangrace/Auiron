use std::ffi::c_void;
use windows::Win32::Networking::WinHttp::WinHttpCloseHandle;
use windows::core::Error;

pub(crate) struct HttpHandle(*mut c_void);

impl HttpHandle {
    pub(crate) fn new(handle: *mut c_void) -> Result<Self, String> {
        if handle.is_null() {
            return Err(Error::from_thread().to_string());
        }

        Ok(Self(handle))
    }

    pub(crate) fn raw(&self) -> *mut c_void {
        self.0
    }
}

impl Drop for HttpHandle {
    fn drop(&mut self) {
        let _ = unsafe { WinHttpCloseHandle(self.0) };
    }
}
