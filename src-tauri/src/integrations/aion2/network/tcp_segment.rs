use super::StreamKey;

#[derive(Clone)]
pub(crate) struct TcpSegment {
    key: StreamKey,
    source_port: u16,
    sequence: u32,
    payload: Vec<u8>,
    timestamp_us: u64,
    reset: bool,
    finished: bool,
}

impl TcpSegment {
    pub(crate) fn new(
        key: StreamKey,
        source_port: u16,
        sequence: u32,
        payload: Vec<u8>,
        timestamp_us: u64,
        reset: bool,
    ) -> Self {
        Self {
            key,
            source_port,
            sequence,
            payload,
            timestamp_us,
            reset,
            finished: false,
        }
    }

    pub(crate) fn key(&self) -> &StreamKey {
        &self.key
    }

    pub(crate) fn source_port(&self) -> u16 {
        self.source_port
    }

    pub(crate) fn sequence(&self) -> u32 {
        self.sequence
    }

    pub(crate) fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub(crate) fn timestamp_us(&self) -> u64 {
        self.timestamp_us
    }

    pub(crate) fn reset(&self) -> bool {
        self.reset
    }

    /// FIN may carry the last bytes of the connection; they are data, unlike RST payloads.
    pub(crate) fn with_finished(mut self, finished: bool) -> Self {
        self.finished = finished;
        self
    }

    pub(crate) fn finished(&self) -> bool {
        self.finished
    }
}
