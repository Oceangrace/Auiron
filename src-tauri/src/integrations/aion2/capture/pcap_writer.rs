use super::CapturedPacket;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

pub(crate) struct PcapWriter {
    file: BufWriter<File>,
    link_type: u32,
}

impl PcapWriter {
    pub(crate) fn open(path: &Path, link_type: u32) -> Result<Self, String> {
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|error| format!("{}: {error}", path.display()))?;

        let mut file = BufWriter::new(file);

        file.write_all(&0xa1b2c3d4u32.to_le_bytes())
            .map_err(|error| error.to_string())?;
        file.write_all(&2u16.to_le_bytes())
            .map_err(|error| error.to_string())?;
        file.write_all(&4u16.to_le_bytes())
            .map_err(|error| error.to_string())?;

        for value in [0, 0, CapturedPacket::MAX_BYTES as u32, link_type] {
            file.write_all(&value.to_le_bytes())
                .map_err(|error| error.to_string())?;
        }

        Ok(Self { file, link_type })
    }

    pub(crate) fn write(&mut self, packet: &CapturedPacket) -> Result<(), String> {
        if packet.link_type() != self.link_type {
            return Err("Cannot mix PCAP link types".into());
        }

        let seconds = u32::try_from(packet.timestamp_us() / 1_000_000)
            .map_err(|_| "PCAP timestamp overflow")?;

        let microseconds = (packet.timestamp_us() % 1_000_000) as u32;

        let length = u32::try_from(packet.data().len()).map_err(|_| "PCAP packet size overflow")?;

        for value in [seconds, microseconds, length, length] {
            self.file
                .write_all(&value.to_le_bytes())
                .map_err(|error| error.to_string())?;
        }

        self.file
            .write_all(packet.data())
            .map_err(|error| error.to_string())
    }

    pub(crate) fn flush(&mut self) -> Result<(), String> {
        self.file.flush().map_err(|error| error.to_string())
    }
}
