pub(crate) struct CapturedPacket {
    timestamp_us: u64,
    link_type: u32,
    data: Vec<u8>,
}

impl CapturedPacket {
    pub(crate) const MAX_BYTES: usize = 262_144;

    pub(crate) fn new(timestamp_us: u64, link_type: u32, data: Vec<u8>) -> Result<Self, String> {
        if data.len() > Self::MAX_BYTES {
            return Err("Captured packet exceeds the size limit".into());
        }

        Ok(Self {
            timestamp_us,
            link_type,
            data,
        })
    }

    pub(crate) fn timestamp_us(&self) -> u64 {
        self.timestamp_us
    }

    pub(crate) fn link_type(&self) -> u32 {
        self.link_type
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.data
    }
}
