use super::source_reader::SourceReader;
use super::source_worker::SourceWorker;
use crate::config::CaptureConfiguration;
use crate::core::game::GameObservationSource;
use crate::core::game::SourceFrame;
use crate::core::game::SourceStatus;
use crate::integrations::aion2::PacketPipeline;
use crate::integrations::aion2::state::GameState;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::Builder;
use std::thread::JoinHandle;

pub(crate) struct GameSource {
    worker: Option<SourceWorker>,
    latest: Arc<Mutex<Arc<SourceFrame>>>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl GameSource {
    pub(crate) fn new(
        configuration: CaptureConfiguration,
        pipeline: PacketPipeline,
        combat_idle_ms: u64,
        recovery_path: PathBuf,
    ) -> Self {
        let snapshot = GameState::new(
            uuid::Uuid::new_v4().to_string(),
            pipeline.capabilities().clone(),
            pipeline.catalog().clone(),
            combat_idle_ms,
        )
        .snapshot(0, SourceStatus::Offline);

        Self {
            worker: Some(SourceWorker::new(
                configuration,
                pipeline,
                combat_idle_ms,
                recovery_path,
            )),
            latest: Arc::new(Mutex::new(Arc::new(SourceFrame::new(snapshot, None)))),
            stop: Arc::new(AtomicBool::new(false)),
            thread: None,
        }
    }

    pub(crate) fn start(&mut self) -> Result<(), String> {
        if self.thread.is_some() {
            return Ok(());
        }

        let worker = self
            .worker
            .take()
            .ok_or("Game source cannot restart after disposal")?;

        let stop = self.stop.clone();

        let latest = self.latest.clone();

        self.thread = Some(
            Builder::new()
                .name("aion2-capture".into())
                .spawn(move || worker.run(stop, latest))
                .map_err(|error| error.to_string())?,
        );

        Ok(())
    }

    pub(crate) fn reader(&self) -> Arc<dyn GameObservationSource> {
        Arc::new(SourceReader::new(self.latest.clone()))
    }

    pub(crate) fn stop(&mut self) {
        self.stop.store(true, Ordering::Release);

        if let Some(thread) = self.thread.take()
            && thread.join().is_err()
        {
            eprintln!("Game capture thread panicked");
        }
    }
}

impl Drop for GameSource {
    fn drop(&mut self) {
        self.stop();
    }
}
