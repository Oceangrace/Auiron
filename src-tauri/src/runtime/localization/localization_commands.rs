use crate::core::localization::LocaleRegistry;
use std::sync::Arc;
use tauri::{State, WebviewWindow};

#[tauri::command]
pub(crate) fn localization_read(
    window: WebviewWindow,
    locales: State<'_, Arc<LocaleRegistry>>,
) -> Result<serde_json::Value, String> {
    if window.label() != "editor" {
        return Err("Editor access required".into());
    }

    Ok(locales.snapshot())
}
