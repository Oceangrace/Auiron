//! Системные голоса и WAV-поток через Microsoft Windows Runtime.

mod runtime_apartment;
mod windows_speech_engine;

use self::runtime_apartment::RuntimeApartment;
pub(crate) use self::windows_speech_engine::WindowsSpeechEngine;
