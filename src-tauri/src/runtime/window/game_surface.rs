use super::game_bounds::GameBounds;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GW_OWNER, GWL_EXSTYLE, GetClassNameW, GetWindow, GetWindowLongPtrW, WS_EX_NOACTIVATE,
    WS_EX_TOOLWINDOW,
};

pub(super) struct GameSurface;

impl GameSurface {
    pub(super) fn read(window: HWND) -> Option<GameBounds> {
        // SAFETY: read-only metadata queries; invalid or destroyed handles are rejected.
        let owned = unsafe { GetWindow(window, GW_OWNER) }.is_ok_and(|owner| !owner.0.is_null());
        let style = unsafe { GetWindowLongPtrW(window, GWL_EXSTYLE) };

        if owned || style & (WS_EX_TOOLWINDOW.0 | WS_EX_NOACTIVATE.0) as isize != 0 {
            return None;
        }

        let mut class = [0u16; 256];
        let length = unsafe { GetClassNameW(window, &mut class) };

        if length <= 0 {
            return None;
        }

        let class = String::from_utf16_lossy(&class[..length as usize]);

        if class.to_ascii_lowercase().contains("splash") {
            return None;
        }

        GameBounds::read(window).filter(|bounds| bounds.fits_surface())
    }
}
