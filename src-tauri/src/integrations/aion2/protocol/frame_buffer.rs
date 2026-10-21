use super::BinaryReader;

pub(crate) struct FrameBuffer {
    pending: Vec<u8>,
}

impl FrameBuffer {
    pub(crate) const MAX_BODY: usize = 1_048_576;
    const MAX_PENDING: usize = 2_097_152;

    pub(crate) fn new() -> Self {
        Self {
            pending: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, bytes: &[u8]) -> Result<Vec<Vec<u8>>, String> {
        if bytes.len() > Self::MAX_PENDING.saturating_sub(self.pending.len()) {
            return Err("Protocol frame buffer limit".into());
        }

        self.pending.extend_from_slice(bytes);

        let mut consumed = 0;

        let mut frames = Vec::new();

        while consumed < self.pending.len() {
            let available = &self.pending[consumed..];

            let mut reader = BinaryReader::new(available);

            let encoded = match reader.varint() {
                Ok(value) => value,
                Err(_) if available.len() < 5 => break,
                Err(error) => return Err(error),
            };

            let length = encoded.checked_sub(4).ok_or("Invalid frame size")?;

            if !(2..=Self::MAX_BODY as u64).contains(&length) {
                return Err("Invalid frame body size".into());
            }

            if length as usize > reader.remaining() {
                break;
            }

            frames.push(reader.bytes(length as usize)?.to_vec());
            consumed += reader.position();
        }

        self.pending.drain(..consumed);

        Ok(frames)
    }

    pub(crate) fn pending_bytes(&self) -> usize {
        self.pending.len()
    }

    pub(crate) fn finish(&self) -> Result<(), String> {
        if !self.pending.is_empty() {
            return Err("Incomplete protocol message boundary".into());
        }

        Ok(())
    }
}
