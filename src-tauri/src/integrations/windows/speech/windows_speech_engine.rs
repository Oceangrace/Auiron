use super::RuntimeApartment;
use crate::core::speech::SpeechAudio;
use crate::core::speech::SpeechEngine;
use crate::core::speech::SpeechRequest;
use crate::core::speech::SpeechVoice;
use windows::Media::SpeechSynthesis::SpeechSynthesizer;
use windows::Storage::Streams::DataReader;
use windows::core::HSTRING;

pub(crate) struct WindowsSpeechEngine;

impl WindowsSpeechEngine {
    pub(crate) fn new() -> Self {
        Self
    }

    fn synthesize(request: &SpeechRequest) -> Result<SpeechAudio, String> {
        let synthesizer = SpeechSynthesizer::new().map_err(Self::error)?;

        let voice = if request.voice().is_empty() {
            SpeechSynthesizer::DefaultVoice().map_err(Self::error)?
        } else {
            SpeechSynthesizer::AllVoices()
                .map_err(Self::error)?
                .into_iter()
                .find(|voice| voice.Id().is_ok_and(|id| id == request.voice()))
                .ok_or("System voice is unavailable. Choose another voice")?
        };

        synthesizer.SetVoice(&voice).map_err(Self::error)?;
        synthesizer
            .Options()
            .map_err(Self::error)?
            .SetSpeakingRate(request.rate() / 100.0)
            .map_err(Self::error)?;

        let stream = synthesizer
            .SynthesizeTextToStreamAsync(&HSTRING::from(request.text()))
            .map_err(Self::error)?
            .join()
            .map_err(Self::error)?;

        let size = stream.Size().map_err(Self::error)?;

        if size > SpeechAudio::MAX_BYTES as u64 {
            return Err("Speech is too long".into());
        }

        let size = u32::try_from(size).map_err(|_| "Speech is too long")?;

        let reader = DataReader::CreateDataReader(&stream).map_err(Self::error)?;

        let loaded = reader
            .LoadAsync(size)
            .map_err(Self::error)?
            .join()
            .map_err(Self::error)?;

        if loaded != size {
            return Err("Could not obtain speech WAV".into());
        }

        let mut bytes = vec![0; size as usize];

        reader.ReadBytes(&mut bytes).map_err(Self::error)?;

        SpeechAudio::try_from(bytes)
    }

    fn error(error: windows::core::Error) -> String {
        format!("Windows speech failed: {error}")
    }
}

impl SpeechEngine for WindowsSpeechEngine {
    fn voices(&self) -> Result<Vec<SpeechVoice>, String> {
        let _apartment = RuntimeApartment::enter()?;

        let mut voices = Vec::new();

        for voice in SpeechSynthesizer::AllVoices().map_err(Self::error)? {
            voices.push(SpeechVoice::new(
                voice.Id().map_err(Self::error)?.to_string(),
                voice.DisplayName().map_err(Self::error)?.to_string(),
                voice.Language().map_err(Self::error)?.to_string(),
            ));
        }

        voices.sort_by(|left, right| left.name().cmp(right.name()));

        Ok(voices)
    }

    fn render(&self, request: &SpeechRequest) -> Result<SpeechAudio, String> {
        request.validate()?;

        let _apartment = RuntimeApartment::enter()?;

        Self::synthesize(request)
    }
}
