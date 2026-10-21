use super::JournalArchive;
use super::JournalFrame;
use super::JournalPreferences;
use super::JournalReport;
use super::JournalRepository;
use crate::core::diagnostics::PerfLog;
use crate::core::game::GameObservationSource;
use crate::integrations::aion2::catalog::RuntimeCatalog;
use std::sync::{Arc, Mutex};
use tauri::async_runtime::spawn_blocking;
use tauri::{State, WebviewWindow};

#[tauri::command]
pub(crate) async fn journal_frame(
    window: WebviewWindow,
    observation_id: Option<String>,
    after_sequence: Option<u64>,
    source: State<'_, Arc<dyn GameObservationSource>>,
    catalog: State<'_, Arc<RuntimeCatalog>>,
) -> Result<JournalFrame, String> {
    if window.label() != "editor" {
        return Err("Unknown journal surface".into());
    }

    let source = Arc::clone(source.inner());
    let catalog = Arc::clone(catalog.inner());

    spawn_blocking(move || {
        PerfLog::measure("journal.frame", || {
            let frame = source.frame()?;

            Ok(JournalFrame::incremental(
                &frame,
                observation_id.as_deref(),
                after_sequence,
                &catalog,
            ))
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub(crate) async fn journal_preferences_read(
    window: WebviewWindow,
    repository: State<'_, Arc<Mutex<JournalRepository>>>,
) -> Result<JournalPreferences, String> {
    if window.label() != "editor" {
        return Err("Unknown journal surface".into());
    }

    let repository = Arc::clone(repository.inner());

    spawn_blocking(move || {
        PerfLog::measure("journal.preferences.read", || {
            repository
                .lock()
                .map_err(|_| "Journal preferences are unavailable")?
                .read()
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub(crate) async fn journal_preferences_save(
    window: WebviewWindow,
    repository: State<'_, Arc<Mutex<JournalRepository>>>,
    preferences: JournalPreferences,
) -> Result<(), String> {
    if window.label() != "editor" {
        return Err("Unknown journal surface".into());
    }

    let repository = Arc::clone(repository.inner());

    spawn_blocking(move || {
        PerfLog::measure("journal.preferences.save", || {
            repository
                .lock()
                .map_err(|_| "Journal preferences are unavailable")?
                .save(preferences)
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub(crate) async fn journal_archive_read(
    window: WebviewWindow,
    archive: State<'_, Arc<Mutex<JournalArchive>>>,
) -> Result<Vec<JournalReport>, String> {
    if window.label() != "editor" {
        return Err("Unknown journal surface".into());
    }

    let archive = Arc::clone(archive.inner());

    spawn_blocking(move || {
        PerfLog::measure("journal.archive.read", || {
            archive
                .lock()
                .map_err(|_| "Combat archive is unavailable")?
                .read()
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub(crate) async fn journal_archive_save(
    window: WebviewWindow,
    archive: State<'_, Arc<Mutex<JournalArchive>>>,
    reports: Vec<JournalReport>,
) -> Result<(), String> {
    if window.label() != "editor" {
        return Err("Unknown journal surface".into());
    }

    let archive = Arc::clone(archive.inner());

    spawn_blocking(move || {
        PerfLog::measure("journal.archive.save", || {
            archive
                .lock()
                .map_err(|_| "Combat archive is unavailable")?
                .save(reports)
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub(crate) async fn journal_export_file(window: WebviewWindow, text: String) -> Result<(), String> {
    use tauri_plugin_dialog::DialogExt;

    if window.label() != "editor" || text.len() > 8 * 1024 * 1024 {
        return Err("Invalid combat export request".into());
    }

    tauri::async_runtime::spawn_blocking(move || {
        let selected = window
            .dialog()
            .file()
            .set_parent(&window)
            .set_file_name("aurion-combat.csv")
            .add_filter("CSV", &["csv"])
            .blocking_save_file();

        let Some(selected) = selected else {
            return Ok(());
        };

        let path = selected.into_path().map_err(|error| error.to_string())?;

        std::fs::write(path, text).map_err(|error| error.to_string())
    })
    .await
    .map_err(|error| error.to_string())?
}
