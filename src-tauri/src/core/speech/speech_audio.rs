pub(crate) struct SpeechAudio {
    bytes: Vec<u8>,
}

impl SpeechAudio {
    pub(crate) const MAX_BYTES: usize = 2_000_000;

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl TryFrom<Vec<u8>> for SpeechAudio {
    type Error = String;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() > Self::MAX_BYTES
            || bytes.get(..4) != Some(b"RIFF")
            || bytes.get(8..12) != Some(b"WAVE")
        {
            return Err("Could not obtain speech WAV".into());
        }

        Ok(Self { bytes })
    }
}
