use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SpeechRequest {
    text: String,
    voice: String,
    rate: f64,
}

impl SpeechRequest {
    pub(crate) fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn voice(&self) -> &str {
        &self.voice
    }

    pub(crate) fn rate(&self) -> f64 {
        self.rate
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.text.trim().is_empty()
            || self.text.chars().count() > 160
            || self.voice.chars().count() > 512
            || !self.rate.is_finite()
            || !(50.0..=200.0).contains(&self.rate)
        {
            return Err("Invalid speech text, voice or rate".into());
        }

        Ok(())
    }
}
