pub(crate) struct DecodeBudget {
    remaining_bytes: usize,
    remaining_messages: usize,
}

impl DecodeBudget {
    /// Per TCP packet, including nested LZ4 bundles.
    const MAX_EXPANDED_BYTES: usize = 8_388_608;
    const MAX_MESSAGES: usize = 4096;

    pub(crate) fn new() -> Self {
        Self {
            remaining_bytes: Self::MAX_EXPANDED_BYTES,
            remaining_messages: Self::MAX_MESSAGES,
        }
    }

    pub(super) fn message(&mut self, bytes: usize) -> Result<(), String> {
        self.remaining_messages = self
            .remaining_messages
            .checked_sub(1)
            .ok_or("Protocol message count limit")?;

        self.expansion(bytes)
    }

    pub(super) fn expansion(&mut self, bytes: usize) -> Result<(), String> {
        self.remaining_bytes = self
            .remaining_bytes
            .checked_sub(bytes)
            .ok_or("Protocol expansion budget exceeded")?;

        Ok(())
    }
}
