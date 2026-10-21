use crate::modules::interfaces::appearance::ConditionAnimation;
use crate::modules::interfaces::appearance::FontWeight;
use crate::modules::interfaces::appearance::GlowMode;
use crate::modules::interfaces::appearance::HexColor;
use crate::modules::interfaces::appearance::OutlineStyle;
use crate::modules::interfaces::appearance::WidgetAnimation;
use crate::modules::interfaces::audio::SoundCue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(tag = "property", deny_unknown_fields)]
pub(crate) enum ConditionChange {
    #[serde(rename = "text")]
    Text {
        #[schemars(length(max = 2048))]
        value: String,
    },
    #[serde(rename = "icon.label")]
    IconLabel {
        #[schemars(length(max = 2048))]
        value: String,
    },
    #[serde(rename = "color")]
    Color { value: HexColor },
    #[serde(rename = "fill_color")]
    FillColor { value: HexColor },
    #[serde(rename = "background")]
    Background { value: HexColor },
    #[serde(rename = "border_color")]
    BorderColor { value: HexColor },
    #[serde(rename = "text.outline-color")]
    TextOutlineColor { value: HexColor },
    #[serde(rename = "icon.glow-color")]
    IconGlowColor { value: HexColor },
    #[serde(rename = "opacity")]
    Opacity {
        #[schemars(range(min = 0, max = 100))]
        value: f64,
    },
    #[serde(rename = "font_size")]
    FontSize {
        #[schemars(range(min = 6, max = 256))]
        value: f64,
    },
    #[serde(rename = "glow")]
    Glow { value: bool },
    #[serde(rename = "desaturate")]
    Desaturate { value: bool },
    #[serde(rename = "icon.swipe")]
    IconSwipe { value: bool },
    #[serde(rename = "bar.spark")]
    BarSpark { value: bool },
    #[serde(rename = "bar.inverse")]
    BarInverse { value: bool },
    #[serde(rename = "animation")]
    Animation { value: WidgetAnimation },
    #[serde(rename = "text.weight")]
    TextWeight { value: FontWeight },
    #[serde(rename = "text.outline")]
    TextOutline { value: OutlineStyle },
    #[serde(rename = "icon.glow")]
    IconGlow { value: GlowMode },
    #[serde(rename = "bar.gradient")]
    BarGradient { value: Option<HexColor> },
    #[serde(rename = "sound")]
    Sound {
        value: SoundCue,
        #[serde(default, rename = "speechText")]
        #[schemars(length(max = 160))]
        speech_text: String,
    },
    #[serde(rename = "animation-config")]
    AnimationConfig { value: ConditionAnimation },
}

impl ConditionChange {
    pub(crate) fn speech_text(&self) -> &str {
        match self {
            Self::Sound { speech_text, .. } => speech_text,
            _ => "",
        }
    }
}
