use super::journal_appearance::JournalAppearance;
use super::journal_panel_layout::JournalPanelLayout;
use super::journal_window::JournalWindow;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct JournalPreferences {
    enabled: bool,
    #[schemars(range(min = 1, max = 100))]
    history_limit: usize,
    #[serde(default)]
    appearance: JournalAppearance,
    #[serde(default)]
    settings_panel: Option<JournalPanelLayout>,
    #[serde(default)]
    analysis_panel: Option<JournalPanelLayout>,
    #[schemars(length(min = 1, max = 8))]
    windows: Vec<JournalWindow>,
}

impl Default for JournalPreferences {
    fn default() -> Self {
        Self {
            enabled: false,
            history_limit: 50,
            appearance: Default::default(),
            settings_panel: None,
            analysis_panel: None,
            windows: vec![JournalWindow::default()],
        }
    }
}

impl JournalPreferences {
    pub(super) fn validate(&self) -> Result<(), String> {
        if self.windows.is_empty()
            || self.windows.len() > 8
            || !(1..=100).contains(&self.history_limit)
        {
            return Err("Invalid journal preferences".into());
        }

        self.appearance.validate()?;

        for panel in [&self.settings_panel, &self.analysis_panel]
            .into_iter()
            .flatten()
        {
            panel.validate()?;
        }

        let mut ids = HashSet::new();

        for window in &self.windows {
            window.validate()?;

            if !ids.insert(window.id()) {
                return Err("Duplicate journal window".into());
            }
        }

        Ok(())
    }
}
