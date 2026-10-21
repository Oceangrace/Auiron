//! Контракты системной речи без зависимости от платформы и интерфейса.

mod speech_audio;
mod speech_engine;
mod speech_request;
mod speech_voice;

pub(crate) use self::speech_audio::SpeechAudio;
pub(crate) use self::speech_engine::SpeechEngine;
pub(crate) use self::speech_request::SpeechRequest;
pub(crate) use self::speech_voice::SpeechVoice;
