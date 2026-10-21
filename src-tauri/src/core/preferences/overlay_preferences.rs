use crate::core::game::CharacterClass;
use crate::core::geometry::OverlayAnchor;
use crate::core::geometry::OverlayPosition;
use crate::core::preferences::PanelLayout;
use crate::core::preferences::ScreenPreset;
use crate::core::preferences::ToolbarLayout;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct OverlayPreferences {
    #[serde(default)]
    pub(crate) class_colors: BTreeMap<CharacterClass, String>,
    #[serde(default)]
    pub(crate) locale: String,
    pub(crate) screen_preset: ScreenPreset,
    pub(crate) ui_scale: f64,
    pub(crate) background_transparency: f64,
    pub(crate) toolbar_anchor: OverlayAnchor,
    pub(crate) toolbar_layout: ToolbarLayout,
    pub(crate) toolbar_position: Option<OverlayPosition>,
    pub(crate) toolbar_show_clock: bool,
    pub(crate) toolbar_show_play_time: bool,
    #[serde(default)]
    pub(crate) toolbar_auto_hide: bool,
    #[serde(default)]
    pub(crate) settings_panel: Option<PanelLayout>,
}

impl Default for OverlayPreferences {
    fn default() -> Self {
        Self {
            class_colors: BTreeMap::new(),
            locale: String::default(),
            screen_preset: ScreenPreset::Auto,
            ui_scale: 100.0,
            background_transparency: 0.0,
            toolbar_anchor: OverlayAnchor::TopCenter,
            toolbar_layout: ToolbarLayout::Horizontal,
            toolbar_position: None,
            toolbar_show_clock: true,
            toolbar_show_play_time: false,
            toolbar_auto_hide: false,
            settings_panel: None,
        }
    }
}

impl OverlayPreferences {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.class_colors.values().any(|color| {
            color.len() != 7
                || !color.starts_with('#')
                || !color.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit)
        }) {
            return Err("Invalid class color".into());
        }

        if self.locale.len() > 63
            || (!self.locale.is_empty() && self.locale.parse::<icu_locale::Locale>().is_err())
        {
            return Err("Invalid locale ID".into());
        }

        if !self.ui_scale.is_finite()
            || !(75.0..=175.0).contains(&self.ui_scale)
            || !self.background_transparency.is_finite()
            || !(0.0..=50.0).contains(&self.background_transparency)
        {
            return Err("Invalid scale or transparency".into());
        }

        if let Some(position) = &self.toolbar_position
            && (!position.x.is_finite()
                || !position.y.is_finite()
                || position.x.abs() > 100_000.0
                || position.y.abs() > 100_000.0)
        {
            return Err("Invalid toolbar position".into());
        }

        if let Some(layout) = &self.settings_panel {
            layout.validate()?;
        }

        Ok(())
    }
}
