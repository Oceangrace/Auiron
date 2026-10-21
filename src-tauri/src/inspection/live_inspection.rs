use crate::config::ApplicationConfiguration;
use crate::core::game::SourceStatus;
use crate::inspection::input_markers::InputMarkers;
use crate::integrations::aion2::PacketPipeline;
use crate::integrations::aion2::TrafficReport;
use crate::integrations::aion2::capture::CaptureRead;
use crate::integrations::aion2::capture::CaptureTransport;
use crate::integrations::aion2::capture::CapturedPacket;
use crate::integrations::aion2::capture::LiveTransport;
use crate::integrations::aion2::capture::PcapWriter;
use crate::integrations::aion2::state::GameState;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub(crate) struct LiveInspection {
    directory: PathBuf,
    pipeline: PacketPipeline,
    state: GameState,
    report: TrafficReport,
    events: BufWriter<File>,
    writers: BTreeMap<u32, PcapWriter>,
    capture_errors: Vec<String>,
    input_markers: Option<InputMarkers>,
}

impl LiveInspection {
    pub(crate) fn record(directory: &Path, seconds: u64) -> Result<String, String> {
        Self::record_with_markers(directory, seconds, false)
    }

    pub(crate) fn record_with_markers(
        directory: &Path,
        seconds: u64,
        markers: bool,
    ) -> Result<String, String> {
        if !(1..=3600).contains(&seconds) {
            return Err("Recording duration must be between 1 and 3600 seconds".into());
        }

        let configuration = ApplicationConfiguration::load()?;

        let pipeline = PacketPipeline::new(
            configuration.protocol()?,
            configuration.npc_index()?,
            configuration.runtime_catalog()?,
        );

        let state = GameState::new(
            uuid::Uuid::new_v4().to_string(),
            pipeline.capabilities().clone(),
            pipeline.catalog().clone(),
            configuration.combat_idle_ms(),
        );

        std::fs::create_dir(directory)
            .map_err(|error| format!("{}: {error}", directory.display()))?;

        let events = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(directory.join("events.jsonl"))
            .map(BufWriter::new)
            .map_err(|error| error.to_string())?;

        let mut inspection = Self {
            directory: directory.to_owned(),
            pipeline,
            state,
            report: Default::default(),
            events,
            writers: BTreeMap::new(),
            capture_errors: Vec::new(),
            input_markers: if markers {
                Some(InputMarkers::open(
                    directory,
                    configuration.capture().process_name(),
                )?)
            } else {
                None
            },
        };

        let mut transport = LiveTransport::new(configuration.capture().clone());

        transport.start()?;

        let result = inspection.pump(&mut transport, seconds);

        let process = transport.process();

        transport.stop();
        result?;

        if let Some(markers) = &mut inspection.input_markers {
            markers.finish(transport.now_us())?;
        }

        for writer in inspection.writers.values_mut() {
            writer.flush()?;
        }

        inspection
            .events
            .flush()
            .map_err(|error| error.to_string())?;

        let snapshot = inspection
            .state
            .snapshot(transport.now_us(), SourceStatus::Live);

        let report = serde_json::to_string_pretty(&serde_json::json!({
            "report": inspection.report.finish(&inspection.pipeline),
            "snapshot": snapshot,
            "captureErrors": inspection.capture_errors,
            "process": process,
        }))
        .map_err(|error| error.to_string())?;

        std::fs::write(directory.join("capture.json"), &report)
            .map_err(|error| error.to_string())?;

        Ok(report)
    }

    fn pump(&mut self, transport: &mut LiveTransport, seconds: u64) -> Result<(), String> {
        let started = Instant::now();

        while started.elapsed() < Duration::from_secs(seconds)
            && !self.directory.join("STOP").exists()
        {
            if let Some(markers) = &mut self.input_markers {
                markers.sample(transport.now_us())?;
            }

            match transport.poll() {
                Ok(CaptureRead::Packet(packet)) => self.packet(&packet)?,
                Ok(CaptureRead::Reset) => self.reset(),
                Ok(CaptureRead::Pending | CaptureRead::Finished) => {
                    std::thread::sleep(Duration::from_millis(5))
                }

                Err(error) => {
                    self.reset();

                    if self.capture_errors.len() < 128 {
                        self.capture_errors.push(error);
                    }

                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }

        Ok(())
    }

    fn packet(&mut self, packet: &CapturedPacket) -> Result<(), String> {
        let link_type = packet.link_type();

        if let Entry::Vacant(entry) = self.writers.entry(link_type) {
            let writer = PcapWriter::open(
                &self.directory.join(format!("link-{link_type}.pcap")),
                link_type,
            )?;

            entry.insert(writer);
        }

        self.writers
            .get_mut(&link_type)
            .ok_or("Missing capture writer")?
            .write(packet)?;

        let update = self.pipeline.push(packet, &[]);

        self.report
            .packet(packet.timestamp_us(), update.reset().is_some());

        for event in update.events() {
            self.state.apply(event, packet.timestamp_us())?;
            self.report.event(event.kind());

            serde_json::to_writer(
                &mut self.events,
                &serde_json::json!({ "atUs": packet.timestamp_us(), "event": event }),
            )
            .map_err(|error| error.to_string())?;
            self.events
                .write_all(b"\n")
                .map_err(|error| error.to_string())?;
        }

        if let Some(reset) = update.reset() {
            self.state.recover(reset);
        }

        Ok(())
    }

    fn reset(&mut self) {
        self.pipeline.reset();
        self.state.reset(uuid::Uuid::new_v4().to_string());
    }
}
