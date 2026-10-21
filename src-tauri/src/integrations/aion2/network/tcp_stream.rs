use super::TcpSegment;
use std::collections::HashMap;

pub(crate) struct TcpStream {
    expected: Option<u32>,
    pending: HashMap<u32, TcpSegment>,
    gap_since_us: Option<u64>,
    pending_bytes: usize,
}

impl TcpStream {
    const MAX_PENDING_SEGMENTS: usize = 128;
    const MAX_PENDING_BYTES: usize = 2_097_152;
    /// A hole older than this is a capture gap, not reordering.
    const MAX_GAP_US: u64 = 2_000_000;

    pub(crate) fn new() -> Self {
        Self {
            expected: None,
            pending: HashMap::new(),
            gap_since_us: None,
            pending_bytes: 0,
        }
    }

    pub(crate) fn push(&mut self, segment: TcpSegment) -> Result<Vec<Vec<u8>>, String> {
        let expected = *self.expected.get_or_insert(segment.sequence());

        let distance = segment.sequence().wrapping_sub(expected) as i32;

        if distance > 0 {
            let length = segment.payload().len();

            let queued = self
                .pending
                .get(&segment.sequence())
                .map(|queued| queued.payload().len());

            // A retransmission can be shorter than the segment already queued at the same
            // sequence; replacing it would drop the queued tail.
            if queued.is_some_and(|queued| queued >= length) {
                return Ok(Vec::new());
            }

            let added = length - queued.unwrap_or(0);

            let since = *self.gap_since_us.get_or_insert(segment.timestamp_us());

            if (queued.is_none() && self.pending.len() >= Self::MAX_PENDING_SEGMENTS)
                || self.pending_bytes.saturating_add(added) > Self::MAX_PENDING_BYTES
                || segment.timestamp_us().saturating_sub(since) > Self::MAX_GAP_US
            {
                return Err("TCP capture gap".into());
            }

            self.pending_bytes += added;
            self.pending.insert(segment.sequence(), segment);

            return Ok(Vec::new());
        }

        let mut output = Vec::new();

        self.append(&segment, &mut output);

        loop {
            let expected = self.expected.ok_or("Missing TCP sequence")?;

            let next = self
                .pending
                .keys()
                .copied()
                .filter(|sequence| sequence.wrapping_sub(expected) as i32 <= 0)
                .min_by_key(|sequence| sequence.wrapping_sub(expected) as i32);

            let Some(sequence) = next else {
                break;
            };

            let segment = self
                .pending
                .remove(&sequence)
                .ok_or("Missing TCP segment")?;

            self.pending_bytes -= segment.payload().len();
            self.append(&segment, &mut output);
        }

        if self.pending.is_empty() {
            self.gap_since_us = None;
        }

        Ok(output)
    }

    fn append(&mut self, segment: &TcpSegment, output: &mut Vec<Vec<u8>>) {
        let Some(expected) = self.expected else {
            return;
        };

        let distance = segment.sequence().wrapping_sub(expected) as i32;

        let overlap = (-(i64::from(distance))).max(0) as usize;

        let bytes = &segment.payload()[overlap.min(segment.payload().len())..];

        if bytes.is_empty() {
            return;
        }

        output.push(bytes.to_vec());
        self.expected = Some(expected.wrapping_add(bytes.len() as u32));
    }
}
