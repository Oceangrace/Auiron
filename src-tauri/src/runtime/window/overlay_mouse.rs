use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::SetLastError;
use windows::Win32::Foundation::WIN32_ERROR;
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_EXSTYLE, GetWindowLongPtrW, SWP_ASYNCWINDOWPOS, SWP_FRAMECHANGED, SWP_NOACTIVATE,
    SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SetWindowLongPtrW, SetWindowPos, WS_EX_LAYERED,
    WS_EX_TRANSPARENT,
};

/// Keep the fullscreen composition surface transparent to native occlusion tracking,
/// even while its visible controls receive mouse input.
pub(super) struct OverlayMouse;

impl OverlayMouse {
    pub(super) fn apply(window: &tauri::WebviewWindow, ignored: bool) -> Result<(), String> {
        let handle = window.hwnd().map_err(|error| error.to_string())?;

        Self::set(HWND(handle.0), ignored)
    }

    fn set(window: HWND, ignored: bool) -> Result<(), String> {
        // SAFETY: read/write only the current process's live overlay HWND. Keep every
        // unrelated flag, including activation, taskbar, topmost and decoration flags.
        let current = unsafe { GetWindowLongPtrW(window, GWL_EXSTYLE) };
        let layered = current | WS_EX_LAYERED.0 as isize;
        let desired = if ignored {
            layered | WS_EX_TRANSPARENT.0 as isize
        } else {
            layered & !(WS_EX_TRANSPARENT.0 as isize)
        };

        if current == desired {
            return Ok(());
        }

        // SAFETY: a zero previous style is valid, so explicitly clear/check last error.
        unsafe { SetLastError(WIN32_ERROR(0)) };
        let previous = unsafe { SetWindowLongPtrW(window, GWL_EXSTYLE, desired) };
        let error = unsafe { GetLastError() };

        if previous == 0 && error.0 != 0 {
            return Err(format!("Overlay mouse style update failed: {}", error.0));
        }

        // SAFETY: update style caching without resizing, reordering or activating any window.
        unsafe {
            SetWindowPos(
                window,
                None,
                0,
                0,
                0,
                0,
                SWP_ASYNCWINDOWPOS
                    | SWP_FRAMECHANGED
                    | SWP_NOACTIVATE
                    | SWP_NOMOVE
                    | SWP_NOSIZE
                    | SWP_NOZORDER,
            )
        }
        .map_err(|error| error.to_string())
    }
}
