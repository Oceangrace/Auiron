use super::game_window::GameWindow;
use std::time::{Duration, Instant};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VK_LBUTTON, VK_MBUTTON, VK_RBUTTON,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CURSOR_SHOWING, CURSORINFO, GetCursorInfo, GetForegroundWindow,
};

// Alt shows the cursor without a panel; camera dragging hides it even inside a panel.
pub(super) struct GameCursor {
    process_name: String,
    window: usize,
    game: bool,
    overlay: bool,
    checked: Option<Instant>,
    alt_held: bool,
    alt_released: Option<Instant>,
    button_held: bool,
    button_released: Option<Instant>,
    panel: bool,
}

impl GameCursor {
    // The game hides the cursor shortly after Alt is released.
    const ALT_GRACE: Duration = Duration::from_millis(200);
    // The game shows the cursor again shortly after a camera drag ends.
    const BUTTON_GRACE: Duration = Duration::from_millis(200);

    pub(super) fn new(process_name: String) -> Self {
        Self {
            process_name,
            window: 0,
            game: false,
            overlay: false,
            checked: None,
            alt_held: false,
            alt_released: None,
            button_held: false,
            button_released: None,
            panel: false,
        }
    }

    pub(super) fn observe(&mut self, alt: bool) -> Option<bool> {
        let shown = self.shown_in_game();

        let now = Instant::now();

        self.track_buttons(Self::buttons_held(), now);
        self.settle(shown, alt, now)
    }

    fn buttons_held() -> bool {
        [VK_LBUTTON, VK_RBUTTON, VK_MBUTTON].iter().any(|button| {
            // SAFETY: read-only query, high bit only.
            (unsafe { GetAsyncKeyState(i32::from(button.0)) }) < 0
        })
    }

    fn track_buttons(&mut self, held: bool, now: Instant) {
        if self.button_held && !held {
            self.button_released = Some(now);
        }

        self.button_held = held;
    }

    pub(super) fn foreground(&self) -> bool {
        self.game || self.overlay
    }

    fn settle(&mut self, shown: Option<bool>, alt: bool, now: Instant) -> Option<bool> {
        if self.alt_held && !alt {
            self.alt_released = Some(now);
        }

        self.alt_held = alt;

        // Our editor owns focus temporarily; it does not prove the game panel closed.
        let shown = shown?;

        let after_alt = self
            .alt_released
            .is_some_and(|released| now.duration_since(released) < Self::ALT_GRACE);

        let dragging = self.button_held
            || self
                .button_released
                .is_some_and(|released| now.duration_since(released) < Self::BUTTON_GRACE);

        // A camera drag hides the cursor without closing the panel.
        if !shown && dragging {
            return None;
        }

        // Alt can expose the cursor in normal play, but cannot dismiss an already observed panel.
        let panel = shown && (self.panel || (!alt && !after_alt));

        if panel == self.panel {
            return None;
        }

        self.panel = panel;

        Some(panel)
    }

    fn shown_in_game(&mut self) -> Option<bool> {
        // SAFETY: read-only query without caller-owned pointers.
        let window = unsafe { GetForegroundWindow() };

        if window.0 as usize != self.window
            || self
                .checked
                .is_none_or(|last| last.elapsed() >= Duration::from_secs(1))
        {
            self.window = window.0 as usize;
            self.overlay = GameWindow::current(window);
            self.game = GameWindow::owns(window, &self.process_name) == Some(true);
            self.checked = Some(Instant::now());
        }

        if self.overlay {
            return None;
        }

        if !self.game {
            return None;
        }

        let mut cursor = CURSORINFO {
            cbSize: std::mem::size_of::<CURSORINFO>() as u32,
            ..Default::default()
        };

        // SAFETY: correctly sized, initialized, writable Windows structure.
        let known = unsafe { GetCursorInfo(&mut cursor) }.is_ok();

        Some(known && cursor.flags.0 & CURSOR_SHOWING.0 != 0)
    }
}
