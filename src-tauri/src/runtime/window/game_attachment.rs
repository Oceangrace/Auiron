use super::game_bounds::GameBounds;
use super::game_surface::GameSurface;
use super::game_window::GameWindow;
use std::time::{Duration, Instant};
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::EnumWindows;
use windows::core::BOOL;

/// Tracks the largest playable game surface after its initial geometry settles.
pub(super) struct GameAttachment {
    process_name: String,
    window: usize,
    checked: Option<Instant>,
    applied: Option<GameBounds>,
    candidate_area: i64,
    bound_window: usize,
    pending: Option<(usize, GameBounds, Instant)>,
}

impl GameAttachment {
    // Revalidate process ownership and discover restarted/recreated game windows.
    const SEARCH_INTERVAL: Duration = Duration::from_secs(1);
    // Skip the initial windowed-to-fullscreen transition before changing HUD scale.
    const SETTLE_TIME: Duration = Duration::from_secs(1);

    pub(super) fn new(process_name: String) -> Self {
        Self {
            process_name,
            window: 0,
            checked: None,
            applied: None,
            candidate_area: 0,
            bound_window: 0,
            pending: None,
        }
    }

    pub(super) fn update(&mut self, overlay: &tauri::WebviewWindow) -> bool {
        if self
            .checked
            .is_none_or(|last| last.elapsed() >= Self::SEARCH_INTERVAL)
        {
            self.window = 0;
            self.candidate_area = 0;
            self.checked = Some(Instant::now());

            // SAFETY: synchronous enumeration; self lives for the full callback sequence.
            let _ = unsafe { EnumWindows(Some(Self::visit), LPARAM(self as *mut Self as isize)) };
        }

        let Some(bounds) = GameSurface::read(HWND(self.window as *mut _)) else {
            self.applied = None;
            self.bound_window = 0;
            self.pending = None;

            return false;
        };

        if !self.settled(bounds, Instant::now()) {
            return false;
        }

        if self.applied != Some(bounds) {
            match bounds.apply(overlay) {
                Ok(()) => self.applied = Some(bounds),
                Err(error) => {
                    eprintln!("Game attachment failed: {error}");

                    return false;
                }
            }
        }

        true
    }

    fn settled(&mut self, bounds: GameBounds, now: Instant) -> bool {
        if self.bound_window == self.window {
            return true;
        }

        if let Some((window, previous, since)) = self.pending
            && window == self.window
            && previous == bounds
        {
            if now.duration_since(since) >= Self::SETTLE_TIME {
                self.bound_window = self.window;
                self.pending = None;

                return true;
            }

            return false;
        }

        self.pending = Some((self.window, bounds, now));

        false
    }

    unsafe extern "system" fn visit(window: HWND, context: LPARAM) -> BOOL {
        // SAFETY: EnumWindows calls synchronously with the exclusive pointer from update.
        let attachment = unsafe { &mut *(context.0 as *mut Self) };

        attachment.consider(window);

        BOOL(1)
    }

    fn consider(&mut self, window: HWND) {
        let Some(bounds) = GameSurface::read(window) else {
            return;
        };

        if bounds.area() > self.candidate_area
            && GameWindow::owns(window, &self.process_name) == Some(true)
        {
            self.window = window.0 as usize;
            self.candidate_area = bounds.area();
        }
    }
}
