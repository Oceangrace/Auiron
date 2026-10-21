use super::CapturedPacket;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Classic little-endian PCAP; microsecond and nanosecond timestamp variants.
pub(crate) struct PcapReader {
    input: BufReader<File>,
    link_type: u32,
    nanoseconds: bool,
    remaining: u64,
}

impl PcapReader {
    pub(crate) fn open(path: &Path) -> Result<Self, String> {
        let file = File::open(path).map_err(|error| format!("{}: {error}", path.display()))?;

        let size = file.metadata().map_err(|error| error.to_string())?.len();

        if !(24..=134_217_728).contains(&size) {
            return Err("Invalid PCAP file size".into());
        }

        let mut input = BufReader::new(file);

        let mut header = [0u8; 24];

        input
            .read_exact(&mut header)
            .map_err(|error| error.to_string())?;

        let nanoseconds = match &header[..4] {
            [0xd4, 0xc3, 0xb2, 0xa1] => false,
            [0x4d, 0x3c, 0xb2, 0xa1] => true,
            _ => return Err("Unsupported PCAP format or byte order".into()),
        };

        if header[4..8] != [2, 0, 4, 0] {
            return Err("Unsupported PCAP version".into());
        }

        let link_type = Self::u32(&header[20..24])?;

        Ok(Self {
            input,
            link_type,
            nanoseconds,
            remaining: size - 24,
        })
    }

    pub(crate) fn next(&mut self) -> Result<Option<CapturedPacket>, String> {
        if self.remaining == 0 {
            return Ok(None);
        }

        if self.remaining < 16 {
            return Err("Truncated PCAP record header".into());
        }

        let mut header = [0u8; 16];

        self.input
            .read_exact(&mut header)
            .map_err(|error| error.to_string())?;
        self.remaining -= 16;

        let seconds = u64::from(Self::u32(&header[..4])?);

        let fraction = u64::from(Self::u32(&header[4..8])?);

        let captured = u64::from(Self::u32(&header[8..12])?);

        let original = u64::from(Self::u32(&header[12..16])?);

        let fraction_limit = if self.nanoseconds {
            1_000_000_000
        } else {
            1_000_000
        };

        if captured != original
            || captured > CapturedPacket::MAX_BYTES as u64
            || captured > self.remaining
            || fraction >= fraction_limit
        {
            return Err("Truncated or invalid PCAP packet".into());
        }

        let mut data = vec![0; captured as usize];

        self.input
            .read_exact(&mut data)
            .map_err(|error| error.to_string())?;
        self.remaining -= captured;

        let timestamp = seconds * 1_000_000
            + if self.nanoseconds {
                fraction / 1000
            } else {
                fraction
            };

        CapturedPacket::new(timestamp, self.link_type, data).map(Some)
    }

    fn u32(bytes: &[u8]) -> Result<u32, String> {
        Ok(u32::from_le_bytes(
            bytes.try_into().map_err(|_| "Invalid PCAP integer")?,
        ))
    }
}
