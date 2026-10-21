use crate::core::diagnostics::PageHeartbeat;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};

pub(crate) struct QuitRequest;

impl QuitRequest {
    pub(crate) fn send(app: &AppHandle) {
        let responsive = app
            .try_state::<Arc<PageHeartbeat>>()
            .is_some_and(|heartbeat| heartbeat.responsive());

        if responsive {
            let _ = app.emit_to("editor", "app:quit-requested", ());

            return;
        }

        app.exit(0);
    }
}
