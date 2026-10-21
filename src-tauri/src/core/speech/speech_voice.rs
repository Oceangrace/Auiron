use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SpeechVoice {
    id: String,
    name: String,
    language: String,
}

impl SpeechVoice {
    pub(crate) fn new(id: String, name: String, language: String) -> Self {
        Self { id, name, language }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}
