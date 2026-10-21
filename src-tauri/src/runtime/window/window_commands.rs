use crate::core::geometry::OverlayRegion;
use crate::runtime::OverlayWindow;
use tauri::{AppHandle, State, WebviewWindow};

#[tauri::command]
pub(crate) fn app_wait_for_game(
    window: WebviewWindow,
    overlay: State<'_, OverlayWindow>,
) -> Result<(), String> {
    if window.label() != "editor" {
        return Err("Editor access required".into());
    }

    overlay.wait_for_game(&window)
}

#[tauri::command]
pub(crate) fn overlay_game_attached(
    window: WebviewWindow,
    overlay: State<'_, OverlayWindow>,
) -> Result<bool, String> {
    if window.label() != "editor" {
        return Err("Editor access required".into());
    }

    Ok(overlay.game_attached())
}

#[tauri::command]
pub(crate) fn overlay_foreground(
    window: WebviewWindow,
    overlay: State<'_, OverlayWindow>,
) -> Result<bool, String> {
    if window.label() != "editor" {
        return Err("Editor access required".into());
    }

    Ok(overlay.foreground())
}

#[tauri::command]
pub(crate) fn overlay_game_panel(
    window: WebviewWindow,
    overlay: State<'_, OverlayWindow>,
) -> Result<bool, String> {
    if window.label() != "editor" {
        return Err("Editor access required".into());
    }

    Ok(overlay.game_panel())
}

#[tauri::command]
pub(crate) fn app_exit(app: AppHandle, window: WebviewWindow) -> Result<(), String> {
    if window.label() != "editor" {
        return Err("Editor access required".into());
    }

    app.exit(0);

    Ok(())
}

#[tauri::command]
pub(crate) fn overlay_regions(
    window: WebviewWindow,
    overlay: State<'_, OverlayWindow>,
    regions: Vec<OverlayRegion>,
) -> Result<(), String> {
    if window.label() != "editor" {
        return Err("Overlay access required".into());
    }

    overlay.set_regions(regions)
}

#[tauri::command]
pub(crate) fn overlay_capture(
    window: WebviewWindow,
    overlay: State<'_, OverlayWindow>,
    captured: bool,
) -> Result<(), String> {
    if window.label() != "editor" {
        return Err("Editor access required".into());
    }

    overlay.set_dragging(captured);

    Ok(())
}

#[tauri::command]
pub(crate) fn app_minimize(window: WebviewWindow) -> Result<(), String> {
    if window.label() != "editor" {
        return Err("Editor access required".into());
    }

    window.minimize().map_err(|error| error.to_string())
}
