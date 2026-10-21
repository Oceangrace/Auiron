use crate::core::speech::SpeechEngine;
use crate::core::speech::SpeechRequest;
use crate::core::speech::SpeechVoice;
use base64::Engine;
use std::sync::Arc;

pub(crate) struct SpeechService {
    engine: Arc<dyn SpeechEngine>,
}

impl SpeechService {
    pub(crate) fn new(engine: Arc<dyn SpeechEngine>) -> Self {
        Self { engine }
    }

    pub(crate) fn voices(&self) -> Result<Vec<SpeechVoice>, String> {
        self.engine.voices()
    }

    pub(crate) fn render(&self, request: &SpeechRequest) -> Result<String, String> {
        request.validate()?;
        let audio = self.engine.render(request)?;

        Ok(base64::engine::general_purpose::STANDARD.encode(audio.bytes()))
    }
}
