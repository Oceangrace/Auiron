use crate::config::ApplicationConfiguration;
use crate::core::game::SourceFrame;
use crate::core::game::SourceStatus;
use crate::integrations::aion2::PacketPipeline;
use crate::integrations::aion2::capture::PcapReader;
use crate::integrations::aion2::state::GameState;
use crate::modules::combat_journal::JournalFrame;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

pub(crate) struct JournalInspection;

impl JournalInspection {
    pub(crate) fn run(recording: &Path, port: u16, output: &Path) -> Result<String, String> {
        if port == 0 {
            return Err("Server port must be positive".into());
        }

        let configuration = ApplicationConfiguration::load()?;

        let mut pipeline = PacketPipeline::new(
            configuration.protocol()?,
            configuration.npc_index()?,
            configuration.runtime_catalog()?,
        );

        let mut state = GameState::new(
            uuid::Uuid::new_v4().to_string(),
            pipeline.capabilities().clone(),
            pipeline.catalog().clone(),
            configuration.combat_idle_ms(),
        );

        let mut input = PcapReader::open(recording)?;

        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(output)
            .map_err(|error| error.to_string())?;

        let mut writer = BufWriter::new(file);

        let mut next_sample = 0;

        let mut count = 0;

        while let Some(packet) = input.next()? {
            let now = packet.timestamp_us();

            let update = pipeline.push(&packet, &[port]);

            for event in update.events() {
                state.apply(event, now)?;
            }

            if let Some(reset) = update.reset() {
                state.recover(reset);
            }

            if now < next_sample {
                continue;
            }

            next_sample = now.saturating_add(500_000);

            let error = (pipeline.network_errors() != 0 || pipeline.stream_errors() != 0)
                .then(|| "Recording contains network or stream gaps".to_owned());

            let source = SourceFrame::new(state.snapshot(now, SourceStatus::Replay), error);

            serde_json::to_writer(&mut writer, &JournalFrame::new(&source, pipeline.catalog()))
                .map_err(|error| error.to_string())?;
            writer.write_all(b"\n").map_err(|error| error.to_string())?;
            count += 1;
        }

        writer.flush().map_err(|error| error.to_string())?;

        Ok(format!("Written {count} native journal frames"))
    }
}
