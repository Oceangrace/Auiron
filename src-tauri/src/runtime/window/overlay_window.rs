use super::alt_input::AltInput;
use super::game_attachment::GameAttachment;
use super::game_cursor::GameCursor;
use super::overlay_mouse::OverlayMouse;
use super::overlay_order::OverlayOrder;
use super::overlay_visibility::OverlayVisibility;
use crate::core::geometry::OverlayRegion;
use crate::runtime::window::OverlayInput;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::utils::config::WindowConfig;
use tauri::{AppHandle, Emitter, Manager, WebviewWindowBuilder};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON};

pub(crate) struct OverlayWindow {
    configuration: WindowConfig,
    process_name: String,
    input: Arc<Mutex<OverlayInput>>,
    running: Arc<AtomicBool>,
    game_panel: Arc<AtomicBool>,
    foreground: Arc<AtomicBool>,
    game_attached: Arc<AtomicBool>,
    waiting_in_tray: Arc<AtomicBool>,
}

impl OverlayWindow {
    pub(crate) fn new(configuration: WindowConfig, process_name: String) -> Self {
        Self {
            configuration,
            process_name,
            input: Arc::new(Mutex::new(OverlayInput::default())),
            running: Arc::new(AtomicBool::new(false)),
            game_panel: Arc::new(AtomicBool::new(false)),
            foreground: Arc::new(AtomicBool::new(false)),
            game_attached: Arc::new(AtomicBool::new(false)),
            waiting_in_tray: Arc::new(AtomicBool::new(false)),
        }
    }

    pub(crate) fn start(&self, app: &AppHandle) -> tauri::Result<()> {
        let window = WebviewWindowBuilder::from_config(app, &self.configuration)?.build()?;

        let monitor = window
            .primary_monitor()?
            .ok_or(tauri::Error::WindowNotFound)?;

        window.set_position(*monitor.position())?;
        window.set_size(*monitor.size())?;
        window.set_ignore_cursor_events(true)?;
        window.show()?;
        self.start_hit_test(app);

        Ok(())
    }

    pub(crate) fn dispose(&self, app: &AppHandle) {
        self.running.store(false, Ordering::Relaxed);

        if let Some(window) = app.get_webview_window(&self.configuration.label) {
            let _ = window.destroy();
        }
    }

    pub(crate) fn set_regions(&self, regions: Vec<OverlayRegion>) -> Result<(), String> {
        self.input
            .lock()
            .map_err(|_| "Overlay input unavailable")?
            .set_regions(regions)
    }

    pub(crate) fn set_dragging(&self, dragging: bool) {
        if let Ok(mut input) = self.input.lock() {
            input.set_dragging(dragging);
        }
    }

    pub(crate) fn game_panel(&self) -> bool {
        self.game_panel.load(Ordering::Relaxed)
    }

    pub(crate) fn foreground(&self) -> bool {
        self.foreground.load(Ordering::Relaxed)
    }

    pub(crate) fn game_attached(&self) -> bool {
        self.game_attached.load(Ordering::Relaxed)
    }

    pub(crate) fn wait_for_game(&self, window: &tauri::WebviewWindow) -> Result<(), String> {
        OverlayVisibility::set(window, false)?;
        self.waiting_in_tray.store(true, Ordering::Relaxed);

        Ok(())
    }

    fn start_hit_test(&self, app: &AppHandle) {
        let app = app.clone();

        let label = self.configuration.label.clone();

        let input = Arc::clone(&self.input);

        let running = Arc::clone(&self.running);

        let game_panel = Arc::clone(&self.game_panel);

        let foreground = Arc::clone(&self.foreground);

        let game_attached = Arc::clone(&self.game_attached);

        let waiting_in_tray = Arc::clone(&self.waiting_in_tray);

        let mut cursor = GameCursor::new(self.process_name.clone());

        let mut attachment = GameAttachment::new(self.process_name.clone());

        running.store(true, Ordering::Relaxed);

        std::thread::spawn(move || {
            let mut alt = AltInput::new();

            if let Err(error) = alt.start() {
                eprintln!("{error}");
            }

            let mut delivered_panel = None;

            let mut delivered_foreground = None;

            let mut delivered_attachment = None;

            while running.load(Ordering::Relaxed) {
                if let Some(window) = app.get_webview_window(&label) {
                    let attached = attachment.update(&window);

                    game_attached.store(attached, Ordering::Relaxed);

                    if attached
                        && waiting_in_tray.load(Ordering::Relaxed)
                        && OverlayVisibility::set(&window, true).is_ok()
                    {
                        waiting_in_tray.store(false, Ordering::Relaxed);
                    }

                    if delivered_attachment != Some(attached)
                        && window.emit_to(&label, "overlay:attached", attached).is_ok()
                    {
                        delivered_attachment = Some(attached);
                    }

                    let ignored = Self::ignores_cursor(&window, &input);

                    if let Err(error) = OverlayMouse::apply(&window, ignored) {
                        eprintln!("{error}");
                    }

                    if let Some(panel) = cursor.observe(alt.observe()) {
                        game_panel.store(panel, Ordering::Relaxed);
                    }

                    let active = attached && cursor.foreground();

                    if let Err(error) = OverlayOrder::apply(&window, active) {
                        eprintln!("{error}");
                    }

                    foreground.store(active, Ordering::Relaxed);

                    if delivered_foreground != Some(active)
                        && window.emit_to(&label, "overlay:foreground", active).is_ok()
                    {
                        delivered_foreground = Some(active);
                    }

                    let panel = game_panel.load(Ordering::Relaxed);

                    if delivered_panel != Some(panel)
                        && window.emit_to(&label, "overlay:game-panel", panel).is_ok()
                    {
                        delivered_panel = Some(panel);
                    }
                }

                std::thread::sleep(Duration::from_millis(16));
            }
        });
    }

    fn ignores_cursor(window: &tauri::WebviewWindow, input: &Mutex<OverlayInput>) -> bool {
        let (dragging, regions) = {
            let Ok(mut input) = input.lock() else {
                return true;
            };

            // Pointer-up may be lost when a control disappears or the webview loses capture.
            // SAFETY: read-only OS button state; no hooks or game process access.
            if unsafe { GetAsyncKeyState(VK_LBUTTON.0 as i32) } >= 0 {
                input.set_dragging(false);
            }

            input.snapshot()
        };

        if dragging {
            return false;
        }

        let (Ok(cursor), Ok(origin), Ok(scale)) = (
            window.cursor_position(),
            window.inner_position(),
            window.scale_factor(),
        ) else {
            return true;
        };

        let x = (cursor.x - f64::from(origin.x)) / scale;

        let y = (cursor.y - f64::from(origin.y)) / scale;

        !regions.iter().any(|region| region.contains(x, y))
    }
}
