use super::journal_border_style::JournalBorderStyle;
use super::journal_font::JournalFont;
use super::journal_header_button::JournalHeaderButton;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct JournalAppearance {
    #[schemars(range(min = 10, max = 32))]
    font_size: u32,
    #[schemars(range(min = 14, max = 64))]
    row_height: u32,
    #[schemars(range(min = 0, max = 1))]
    opacity: f64,
    #[schemars(range(min = 0, max = 4))]
    border_width: u32,
    #[schemars(range(min = 0, max = 24))]
    padding: u32,
    #[schemars(range(min = 0, max = 100))]
    header_opacity: u32,
    #[schemars(range(min = 20, max = 48))]
    header_height: u32,
    #[schemars(range(min = 8, max = 24))]
    header_font_size: u32,
    #[schemars(range(min = 0, max = 100))]
    row_background_opacity: u32,
    #[schemars(range(min = 0, max = 12))]
    row_gap: u32,
    #[schemars(range(min = 0, max = 16))]
    row_padding: u32,
    #[schemars(range(min = 0, max = 12))]
    row_radius: u32,
    #[schemars(range(min = 0, max = 100))]
    bar_opacity: u32,
    #[schemars(range(min = 0, max = 3))]
    decimals: u32,
    #[schemars(range(min = 0, max = 2000))]
    animation_ms: u32,
    #[schemars(length(min = 7, max = 7))]
    accent: String,
    #[schemars(length(min = 7, max = 7))]
    background_color: String,
    #[schemars(length(min = 7, max = 7))]
    border_color: String,
    #[schemars(length(min = 7, max = 7))]
    header_color: String,
    #[schemars(length(min = 7, max = 7))]
    header_text_color: String,
    #[schemars(length(min = 7, max = 7))]
    row_background_color: String,
    #[schemars(length(min = 7, max = 7))]
    text_color: String,
    #[schemars(length(min = 7, max = 7))]
    value_color: String,
    #[schemars(length(min = 7, max = 7))]
    total_color: String,
    custom_colors: bool,
    class_colors: bool,
    bars: bool,
    compact_numbers: bool,
    show_total: bool,
    show_header: bool,
    bar_gradient: bool,
    show_rank: bool,
    show_percent: bool,
    border_style: JournalBorderStyle,
    font: JournalFont,
    #[schemars(length(max = 7))]
    header_buttons: Vec<JournalHeaderButton>,
    #[schemars(range(min = 12, max = 32))]
    header_button_size: u32,
    header_buttons_on_hover: bool,
}

impl Default for JournalAppearance {
    fn default() -> Self {
        Self {
            font_size: 11,
            row_height: 22,
            opacity: 0.9,
            border_width: 1,
            padding: 4,
            header_opacity: 100,
            header_height: 44,
            header_font_size: 13,
            row_background_opacity: 100,
            row_gap: 2,
            row_padding: 5,
            row_radius: 0,
            bar_opacity: 80,
            decimals: 1,
            animation_ms: 300,
            accent: "#42bbb9".into(),
            background_color: "#192123".into(),
            border_color: "#3b4a4d".into(),
            header_color: "#202a2c".into(),
            header_text_color: "#f5f5f5".into(),
            row_background_color: "#20292b".into(),
            text_color: "#f5f5f5".into(),
            value_color: "#f5f5f5".into(),
            total_color: "#adbec3".into(),
            custom_colors: false,
            class_colors: true,
            bars: true,
            compact_numbers: true,
            show_total: true,
            show_header: true,
            bar_gradient: false,
            show_rank: true,
            show_percent: true,
            border_style: Default::default(),
            font: Default::default(),
            header_buttons: JournalHeaderButton::defaults(),
            header_button_size: 20,
            header_buttons_on_hover: false,
        }
    }
}

impl JournalAppearance {
    pub(super) fn validate(&self) -> Result<(), String> {
        if !(10..=32).contains(&self.font_size)
            || !(14..=64).contains(&self.row_height)
            || !self.opacity.is_finite()
            || !(0.0..=1.0).contains(&self.opacity)
            || !(0..=4).contains(&self.border_width)
            || !(0..=24).contains(&self.padding)
            || !(0..=100).contains(&self.header_opacity)
            || !(20..=48).contains(&self.header_height)
            || !(8..=24).contains(&self.header_font_size)
            || !(0..=100).contains(&self.row_background_opacity)
            || !(0..=12).contains(&self.row_gap)
            || !(0..=16).contains(&self.row_padding)
            || !(0..=12).contains(&self.row_radius)
            || !(0..=100).contains(&self.bar_opacity)
            || !(0..=3).contains(&self.decimals)
            || !(0..=2000).contains(&self.animation_ms)
            || !(12..=32).contains(&self.header_button_size)
        {
            return Err("Invalid journal appearance".into());
        }

        let mut buttons = HashSet::new();

        if !self
            .header_buttons
            .iter()
            .all(|button| buttons.insert(*button))
        {
            return Err("Duplicate journal header button".into());
        }

        for color in [
            &self.accent,
            &self.background_color,
            &self.border_color,
            &self.header_color,
            &self.header_text_color,
            &self.row_background_color,
            &self.text_color,
            &self.value_color,
            &self.total_color,
        ] {
            if color.len() != 7
                || !color.starts_with('#')
                || !color.chars().skip(1).all(|value| value.is_ascii_hexdigit())
            {
                return Err("Invalid journal color".into());
            }
        }

        Ok(())
    }
}
