use crate::modules::interfaces::appearance::ConditionAnimation;
use crate::modules::interfaces::appearance::FontWeight;
use crate::modules::interfaces::appearance::GlowMode;
use crate::modules::interfaces::appearance::HexColor;
use crate::modules::interfaces::appearance::OutlineStyle;
use crate::modules::interfaces::evaluation::PreviewVisual;
use crate::modules::interfaces::rules::ConditionChange;

pub(crate) struct VisualChanges;

impl VisualChanges {
    pub(crate) fn color(value: &str) -> Result<(), String> {
        HexColor::try_from(value.to_owned()).map(|_| ())
    }

    fn number(value: f64, min: f64, max: f64) -> Result<f64, String> {
        if !value.is_finite() || !(min..=max).contains(&value) {
            return Err("Value is out of range".into());
        }

        Ok(value)
    }

    pub(crate) fn apply(
        visual: &mut PreviewVisual,
        change: &ConditionChange,
    ) -> Result<(), String> {
        match change {
            ConditionChange::Text { value } | ConditionChange::IconLabel { value } => {
                if value.chars().count() > 2048 {
                    return Err("Condition text is too long".into());
                }

                visual.appearance.text.clone_from(value);
            }

            ConditionChange::FillColor { value } => {
                visual.style.fill_color = value.as_str().to_owned()
            }

            ConditionChange::Color { value } => visual.appearance.color = value.as_str().to_owned(),
            ConditionChange::Background { value } => {
                visual.appearance.background = value.as_str().to_owned()
            }

            ConditionChange::BorderColor { value } => {
                visual.appearance.border_color = value.as_str().to_owned()
            }

            ConditionChange::TextOutlineColor { value } => {
                visual.style.outline_color = value.as_str().to_owned()
            }

            ConditionChange::IconGlowColor { value } => {
                visual.style.glow_color = value.as_str().to_owned()
            }

            ConditionChange::Opacity { value } => {
                visual.appearance.opacity = Self::number(*value, 0.0, 100.0)? / 100.0
            }

            ConditionChange::FontSize { value } => {
                visual.appearance.font_size = Self::number(*value, 6.0, 256.0)?
            }

            ConditionChange::Glow { value } => visual.style.glow = *value,
            ConditionChange::Desaturate { value } => visual.style.desaturate = *value,
            ConditionChange::Animation { value } => visual.style.animation = *value,
            ConditionChange::TextWeight { value } => visual.style.bold = *value == FontWeight::Bold,
            ConditionChange::TextOutline { value } => {
                visual.style.outline = *value != OutlineStyle::None;
                visual.style.outline_width = match value {
                    OutlineStyle::None => 0.0,
                    OutlineStyle::Thin => 1.0,
                    OutlineStyle::Thick => 2.0,
                };
            }

            ConditionChange::IconSwipe { value } => visual.style.swipe = *value,
            ConditionChange::IconGlow { value } => {
                visual.style.glow_mode = *value;
                visual.style.glow = *value != GlowMode::None;
            }

            ConditionChange::BarGradient { value } => {
                visual.style.gradient = value.as_ref().map(|color| color.as_str().to_owned())
            }

            ConditionChange::BarSpark { value } => visual.style.spark = *value,
            ConditionChange::BarInverse { value } => visual.style.bar_inverse = *value,
            ConditionChange::Sound { value, speech_text } => {
                if speech_text.chars().count() > 160 {
                    return Err("Phrase exceeds 160 characters".into());
                }

                visual.speech_text.clone_from(speech_text);
                visual.sound = *value;
            }

            ConditionChange::AnimationConfig { value } => Self::animation(visual, value)?,
        }

        Ok(())
    }

    fn animation(visual: &mut PreviewVisual, value: &ConditionAnimation) -> Result<(), String> {
        if value.path.len() > 128 {
            return Err("Animation field is too long".into());
        }

        visual.style.period_ms = Self::number(value.period_ms, 100.0, 10000.0)?;
        visual.style.animation_intensity = Self::number(value.intensity, 0.0, 100.0)?;
        visual.style.animation_threshold = Self::number(value.threshold_seconds, 0.0, 3600.0)?;
        visual.style.animation_duration = Self::number(value.duration_ms, 100.0, 60000.0)?;
        visual.style.animation = value.preset;
        visual.style.animation_when = value.when;
        visual.style.animation_color = value.color.as_str().to_owned();
        visual.style.animation_path.clone_from(&value.path);
        visual.style.animation_change = value.change;

        Ok(())
    }
}
