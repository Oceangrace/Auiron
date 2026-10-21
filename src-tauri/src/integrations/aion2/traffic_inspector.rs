use super::PacketPipeline;
use super::TrafficReport;
use super::capture::PcapReader;
use super::recorded_event::RecordedEvent;
use crate::config::ApplicationConfiguration;
use crate::core::game::SourceStatus;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

pub(crate) struct TrafficInspector;

impl TrafficInspector {
    pub(crate) fn inspect(
        path: &Path,
        server_port: u16,
        events_path: Option<&Path>,
    ) -> Result<String, String> {
        if server_port == 0 {
            return Err("Server port must be positive".into());
        }

        let configuration = ApplicationConfiguration::load()?;
        let mut pipeline = PacketPipeline::new(
            configuration.protocol()?,
            configuration.npc_index()?,
            configuration.runtime_catalog()?,
        );
        let mut input = PcapReader::open(path)?;
        let mut report = TrafficReport::default();
        let mut state = super::state::GameState::new(
            uuid::Uuid::new_v4().to_string(),
            pipeline.capabilities().clone(),
            pipeline.catalog().clone(),
            configuration.combat_idle_ms(),
        );
        let mut now_us = 0;
        let mut events_file = events_path
            .map(|path| {
                OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(path)
                    .map(BufWriter::new)
                    .map_err(|error| format!("{}: {error}", path.display()))
            })
            .transpose()?;

        while let Some(packet) = input.next()? {
            let update = pipeline.push(&packet, &[server_port]);

            now_us = packet.timestamp_us();

            report.packet(now_us, update.reset().is_some());

            for event in update.events() {
                state.apply(event, now_us)?;

                report.event(event.kind());

                if let Some(file) = &mut events_file {
                    serde_json::to_writer(
                        &mut *file,
                        &RecordedEvent::new(packet.timestamp_us(), event),
                    )
                    .map_err(|error| error.to_string())?;
                    file.write_all(b"\n").map_err(|error| error.to_string())?;
                }
            }

            if let Some(reset) = update.reset() {
                state.recover(reset);
            }
        }

        if let Some(file) = &mut events_file {
            file.flush().map_err(|error| error.to_string())?;
        }

        let snapshot = state.snapshot(now_us, SourceStatus::Replay);

        serde_json::to_string_pretty(
            &serde_json::json!({ "report": report.finish(&pipeline), "snapshot": snapshot }),
        )
        .map_err(|error| error.to_string())
    }
}
