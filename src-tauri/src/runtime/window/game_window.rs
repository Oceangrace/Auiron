use std::path::Path;
use windows::Win32::Foundation::{CloseHandle, HWND};
use windows::Win32::System::Threading::{
    GetCurrentProcessId, OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
    QueryFullProcessImageNameW,
};
use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
use windows::core::PWSTR;

pub(crate) struct GameWindow;

impl GameWindow {
    pub(super) fn current(window: HWND) -> bool {
        let mut pid = 0;

        // SAFETY: read-only queries and a valid writable process ID output.
        unsafe { GetWindowThreadProcessId(window, Some(&mut pid)) };

        // SAFETY: process ID query takes no pointers.
        pid != 0 && pid == unsafe { GetCurrentProcessId() }
    }

    pub(crate) fn owns(window: HWND, expected: &str) -> Option<bool> {
        if window.0.is_null() {
            return None;
        }

        let mut pid = 0;

        // SAFETY: valid writable process ID output; a missing window yields zero.
        unsafe { GetWindowThreadProcessId(window, Some(&mut pid)) };

        // SAFETY: query-only handle; closed after the single image-path query below.
        let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) }.ok()?;

        let mut buffer = [0u16; 32768];

        let mut length = buffer.len() as u32;

        // SAFETY: writable buffer of the supplied length and live query-only handle.
        let result = unsafe {
            QueryFullProcessImageNameW(
                process,
                PROCESS_NAME_WIN32,
                PWSTR(buffer.as_mut_ptr()),
                &mut length,
            )
        };

        // SAFETY: this method uniquely owns the query handle, including on query failure.
        let _ = unsafe { CloseHandle(process) };

        result.ok()?;

        let path = String::from_utf16(buffer.get(..length as usize)?).ok()?;

        Some(
            Path::new(&path)
                .file_name()?
                .to_str()?
                .eq_ignore_ascii_case(expected),
        )
    }
}
