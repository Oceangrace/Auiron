use crate::modules::interfaces::audio::SoundCue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub(crate) struct WidgetSounds {
    pub(in crate::modules::interfaces) show: SoundCue,
    pub(in crate::modules::interfaces) hide: SoundCue,
    #[schemars(range(min = 0, max = 100))]
    pub(in crate::modules::interfaces) volume: f64,
    #[schemars(length(max = 160))]
    pub(in crate::modules::interfaces) show_text: String,
    #[schemars(length(max = 160))]
    pub(in crate::modules::interfaces) hide_text: String,
    #[schemars(length(max = 512))]
    pub(in crate::modules::interfaces) voice: String,
    #[schemars(range(min = 50, max = 200))]
    pub(in crate::modules::interfaces) voice_rate: f64,
}

impl Default for WidgetSounds {
    fn default() -> Self {
        Self {
            show: SoundCue::None,
            hide: SoundCue::None,
            volume: 70.0,
            show_text: String::new(),
            hide_text: String::new(),
            voice: String::new(),
            voice_rate: 100.0,
        }
    }
}
