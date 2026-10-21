use super::SpeechAudio;
use super::SpeechRequest;
use super::SpeechVoice;

pub(crate) trait SpeechEngine: Send + Sync {
    fn voices(&self) -> Result<Vec<SpeechVoice>, String>;

    fn render(&self, request: &SpeechRequest) -> Result<SpeechAudio, String>;
}
