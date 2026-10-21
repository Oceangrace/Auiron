use crate::core::icons::IconCache;
use crate::core::icons::IconDownload;
use crate::core::icons::IconLoader;
use crate::core::icons::IconTransport;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use tauri::AppHandle;
use tauri::Emitter;

pub(crate) struct IconService;

impl IconService {
    pub(crate) fn start(
        app: AppHandle,
        cache: Arc<IconCache>,
        plan: Vec<IconDownload>,
        transport: Arc<dyn IconTransport>,
        log: PathBuf,
    ) {
        let failure_log = log.clone();
        let result = thread::Builder::new()
            .name("icon-cache".into())
            .spawn(move || {
                let result = IconLoader::run(&cache, &plan, transport.as_ref(), &|files| {
                    let _ = app.emit("icons:changed", files);
                });

                if let Err(error) = result {
                    Self::log(&log, &error);
                }
            });

        if let Err(error) = result {
            Self::log(&failure_log, &error.to_string());
        }
    }

    fn log(path: &PathBuf, error: &str) {
        if let Ok(mut output) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(output, "icons: {}", error.replace(['\r', '\n'], " "));
        }
    }
}
