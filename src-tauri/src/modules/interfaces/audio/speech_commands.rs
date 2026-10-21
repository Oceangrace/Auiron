use crate::core::speech::SpeechRequest;
use crate::core::speech::SpeechVoice;
use crate::modules::interfaces::SpeechService;
use std::sync::Arc;
use tauri::{State, WebviewWindow};

#[tauri::command]
pub(crate) async fn interfaces_speech_voices(
    window: WebviewWindow,
    service: State<'_, Arc<SpeechService>>,
) -> Result<Vec<SpeechVoice>, String> {
    if window.label() != "editor" {
        return Err("Editor access required".into());
    }

    let service = Arc::clone(&service);

    tauri::async_runtime::spawn_blocking(move || service.voices())
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub(crate) async fn interfaces_speech_render(
    window: WebviewWindow,
    request: SpeechRequest,
    service: State<'_, Arc<SpeechService>>,
) -> Result<String, String> {
    if window.label() != "editor" {
        return Err("Editor access required".into());
    }

    request.validate()?;
    let service = Arc::clone(&service);

    tauri::async_runtime::spawn_blocking(move || service.render(&request))
        .await
        .map_err(|error| error.to_string())?
}
