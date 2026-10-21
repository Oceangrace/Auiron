pub(crate) struct BinaryReader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> BinaryReader<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    pub(crate) fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }

    pub(crate) fn position(&self) -> usize {
        self.position
    }

    pub(crate) fn fork(&self) -> Self {
        Self::new(&self.bytes[self.position..])
    }

    pub(crate) fn seek(&mut self, position: usize) -> Result<(), String> {
        if position > self.bytes.len() {
            return Err("Binary position out of range".into());
        }

        self.position = position;

        Ok(())
    }

    pub(crate) fn bytes(&mut self, count: usize) -> Result<&'a [u8], String> {
        let end = self
            .position
            .checked_add(count)
            .ok_or("Binary position overflow")?;

        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or("Truncated protocol message")?;

        self.position = end;

        Ok(bytes)
    }

    pub(crate) fn skip(&mut self, count: usize) -> Result<(), String> {
        self.bytes(count)?;

        Ok(())
    }

    pub(crate) fn byte(&mut self) -> Result<u8, String> {
        Ok(self.bytes(1)?[0])
    }

    pub(crate) fn u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_le_bytes(
            self.bytes(2)?.try_into().map_err(|_| "Invalid u16")?,
        ))
    }

    pub(crate) fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(
            self.bytes(4)?.try_into().map_err(|_| "Invalid u32")?,
        ))
    }

    pub(crate) fn i32(&mut self) -> Result<i32, String> {
        Ok(i32::from_le_bytes(
            self.bytes(4)?.try_into().map_err(|_| "Invalid i32")?,
        ))
    }

    pub(crate) fn i64(&mut self) -> Result<i64, String> {
        Ok(i64::from_le_bytes(
            self.bytes(8)?.try_into().map_err(|_| "Invalid i64")?,
        ))
    }

    pub(crate) fn varint(&mut self) -> Result<u64, String> {
        let mut value = 0u64;

        for shift in (0..35).step_by(7) {
            let byte = self.byte()?;

            value |= u64::from(byte & 127) << shift;

            if byte & 128 == 0 {
                return Ok(value);
            }
        }

        Err("Protocol varint exceeds five bytes".into())
    }

    pub(crate) fn server_time(&mut self) -> Result<Option<u64>, String> {
        let value = self.i64()?;

        if matches!(value, i64::MAX | 4_102_444_800_000) {
            return Ok(None);
        }

        u64::try_from(value)
            .map(Some)
            .map_err(|_| "Negative server time".into())
    }
}
