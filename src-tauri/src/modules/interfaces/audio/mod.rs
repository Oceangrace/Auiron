//! Настройки сигналов и системный синтез речи.

mod sound_cue;
pub(crate) mod speech_commands;
mod speech_service;
mod widget_sounds;

pub(crate) use self::sound_cue::SoundCue;
pub(crate) use self::speech_service::SpeechService;
pub(crate) use self::widget_sounds::WidgetSounds;
