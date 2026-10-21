//! Прозрачное окно, обработка мыши и оконные команды.

mod alt_input;
mod alt_keys;
mod game_attachment;
mod game_bounds;
mod game_cursor;
mod game_surface;
mod game_window;
mod overlay_input;
mod overlay_mouse;
mod overlay_order;
mod overlay_visibility;
mod overlay_window;
pub(crate) mod window_commands;

pub(crate) use self::game_window::GameWindow;
pub(crate) use self::overlay_input::OverlayInput;
pub(crate) use self::overlay_window::OverlayWindow;
