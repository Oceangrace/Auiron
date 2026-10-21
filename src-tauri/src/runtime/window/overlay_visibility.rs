use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    SWP_ASYNCWINDOWPOS, SWP_HIDEWINDOW, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER,
    SWP_SHOWWINDOW, SetWindowPos,
};

/// Temporarily hide the overlay while waiting for the game, without activation on return.
/// Keep Tao's normal visible state: its tray show/focus and minimize commands still work.
pub(super) struct OverlayVisibility;

impl OverlayVisibility {
    pub(super) fn set(window: &tauri::WebviewWindow, visible: bool) -> Result<(), String> {
        let handle = window.hwnd().map_err(|error| error.to_string())?;

        Self::apply(HWND(handle.0), visible)
    }

    fn apply(window: HWND, visible: bool) -> Result<(), String> {
        let visibility = if visible {
            SWP_SHOWWINDOW
        } else {
            SWP_HIDEWINDOW
        };

        // SAFETY: only our overlay HWND; defer to its owner thread, retain geometry,
        // z-order, keyboard focusability, and the foreground application's focus.
        unsafe {
            SetWindowPos(
                window,
                None,
                0,
                0,
                0,
                0,
                SWP_ASYNCWINDOWPOS
                    | SWP_NOACTIVATE
                    | SWP_NOMOVE
                    | SWP_NOSIZE
                    | SWP_NOZORDER
                    | visibility,
            )
        }
        .map_err(|error| error.to_string())
    }
}
