use super::CaptureRead;
use super::CaptureTransport;
use super::CapturedPacket;
use super::PcapReader;
use crate::core::game::SourceStatus;
use std::path::PathBuf;
use std::time::Instant;

pub(crate) struct ReplayTransport {
    path: PathBuf,
    speed: f64,
    ports: Vec<u16>,
    reader: Option<PcapReader>,
    pending: Option<CapturedPacket>,
    started: Option<Instant>,
    first_us: u64,
    last_us: u64,
    reset_pending: bool,
}

impl ReplayTransport {
    pub(crate) fn new(path: PathBuf, speed: f64, ports: Vec<u16>) -> Result<Self, String> {
        if !speed.is_finite()
            || speed <= 0.0
            || speed > 100.0
            || ports.is_empty()
            || ports.contains(&0)
        {
            return Err("Invalid replay speed or server ports".into());
        }

        Ok(Self {
            path,
            speed,
            ports,
            reader: None,
            pending: None,
            started: None,
            first_us: 0,
            last_us: 0,
            reset_pending: false,
        })
    }
}

impl CaptureTransport for ReplayTransport {
    fn start(&mut self) -> Result<(), String> {
        let mut reader = PcapReader::open(&self.path)?;

        let first = reader.next()?.ok_or("Recording has no packets")?;

        self.first_us = first.timestamp_us();
        self.last_us = self.first_us;
        self.pending = Some(first);
        self.reader = Some(reader);
        self.started = Some(Instant::now());
        self.reset_pending = true;

        Ok(())
    }

    fn poll(&mut self) -> Result<CaptureRead, String> {
        if self.reset_pending {
            self.reset_pending = false;

            return Ok(CaptureRead::Reset);
        }

        let Some(packet) = self.pending.as_ref() else {
            return Ok(CaptureRead::Finished);
        };

        if packet.timestamp_us() > self.now_us() {
            return Ok(CaptureRead::Pending);
        }

        let packet = self.pending.take().ok_or("Missing replay packet")?;

        self.last_us = self.last_us.max(packet.timestamp_us());
        self.pending = self
            .reader
            .as_mut()
            .ok_or("Replay is not started")?
            .next()?;

        Ok(CaptureRead::Packet(packet))
    }

    fn now_us(&self) -> u64 {
        let elapsed = self.started.map_or(0.0, |started| {
            started.elapsed().as_secs_f64() * 1_000_000.0 * self.speed
        });

        let now = self.first_us.saturating_add(elapsed as u64);

        if self.pending.is_none() {
            self.last_us
        } else {
            now
        }
    }

    fn mode(&self) -> SourceStatus {
        SourceStatus::Replay
    }

    fn server_ports(&self) -> &[u16] {
        &self.ports
    }

    fn stop(&mut self) {
        self.reader = None;
        self.pending = None;
        self.started = None;
    }
}
