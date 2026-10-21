use windows::Win32::Foundation::{HWND, POINT, RECT};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::UI::WindowsAndMessaging::{
    GetClientRect, IsIconic, IsWindowVisible, SWP_ASYNCWINDOWPOS, SWP_NOACTIVATE, SWP_NOZORDER,
    SetWindowPos,
};

/// Physical screen coordinates of the game's drawable client area.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct GameBounds {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl GameBounds {
    // Conservative launch-window filter, not a requirement for fullscreen play.
    const MIN_SURFACE_WIDTH: i32 = 800;
    const MIN_SURFACE_HEIGHT: i32 = 450;

    pub(super) fn read(window: HWND) -> Option<Self> {
        // SAFETY: read-only queries; stale handles fail without dereferencing them.
        if !unsafe { IsWindowVisible(window) }.as_bool() || unsafe { IsIconic(window) }.as_bool() {
            return None;
        }

        let mut rect = RECT::default();

        let mut origin = POINT::default();

        // SAFETY: both output structures are initialized and writable.
        unsafe { GetClientRect(window, &mut rect) }.ok()?;

        if !unsafe { ClientToScreen(window, &mut origin) }.as_bool() {
            return None;
        }

        Self::from_client(rect, origin)
    }

    fn from_client(rect: RECT, origin: POINT) -> Option<Self> {
        let width = rect.right.checked_sub(rect.left)?;

        let height = rect.bottom.checked_sub(rect.top)?;

        (width > 0 && height > 0).then_some(Self {
            x: origin.x,
            y: origin.y,
            width,
            height,
        })
    }

    pub(super) fn area(self) -> i64 {
        i64::from(self.width) * i64::from(self.height)
    }

    pub(super) fn fits_surface(self) -> bool {
        self.width >= Self::MIN_SURFACE_WIDTH && self.height >= Self::MIN_SURFACE_HEIGHT
    }

    pub(super) fn apply(self, overlay: &tauri::WebviewWindow) -> Result<(), String> {
        let handle = overlay.hwnd().map_err(|error| error.to_string())?;

        // SAFETY: reposition only our own window, asynchronously on its owning thread.
        // Preserve focus and z-order; never reparent or modify the game's window.
        unsafe {
            SetWindowPos(
                HWND(handle.0),
                None,
                self.x,
                self.y,
                self.width,
                self.height,
                SWP_ASYNCWINDOWPOS | SWP_NOACTIVATE | SWP_NOZORDER,
            )
        }
        .map_err(|error| error.to_string())
    }
}
