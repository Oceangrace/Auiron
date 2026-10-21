use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_EXSTYLE, GetForegroundWindow, GetWindowLongPtrW, HWND_BOTTOM, HWND_NOTOPMOST, HWND_TOPMOST,
    SWP_ASYNCWINDOWPOS, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOSIZE, SetWindowPos,
    WS_EX_TOPMOST,
};

pub(super) struct OverlayOrder;

impl OverlayOrder {
    pub(super) fn apply(window: &tauri::WebviewWindow, game_active: bool) -> Result<(), String> {
        let handle = window.hwnd().map_err(|error| error.to_string())?;

        Self::set(HWND(handle.0), game_active)
    }

    fn set(window: HWND, topmost: bool) -> Result<(), String> {
        // SAFETY: read only our live overlay window's extended style.
        let current = unsafe { GetWindowLongPtrW(window, GWL_EXSTYLE) };

        if (current & WS_EX_TOPMOST.0 as isize != 0) == topmost {
            return Ok(());
        }

        let order = if topmost {
            HWND_TOPMOST
        } else if unsafe { GetForegroundWindow() } != window {
            // NOTOPMOST alone would put us above the app the user just switched to.
            HWND_BOTTOM
        } else {
            HWND_NOTOPMOST
        };

        // SAFETY: change only our HWND's stacking band, without activation, geometry,
        // visibility or input style changes. Dispatch to the window's owner thread.
        unsafe {
            SetWindowPos(
                window,
                Some(order),
                0,
                0,
                0,
                0,
                SWP_ASYNCWINDOWPOS | SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE | SWP_NOOWNERZORDER,
            )
        }
        .map_err(|error| error.to_string())
    }
}
