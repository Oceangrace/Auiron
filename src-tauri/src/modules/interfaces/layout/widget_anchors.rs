use crate::modules::interfaces::layout::AnchorMode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct WidgetAnchors {
    pub(in crate::modules::interfaces) horizontal: AnchorMode,
    pub(in crate::modules::interfaces) vertical: AnchorMode,
    pub(in crate::modules::interfaces) right: f64,
    pub(in crate::modules::interfaces) bottom: f64,
    pub(in crate::modules::interfaces) width_percent: Option<f64>,
    pub(in crate::modules::interfaces) height_percent: Option<f64>,
}

impl WidgetAnchors {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if !self.right.is_finite()
            || !self.bottom.is_finite()
            || self.right.abs() > 100_000.0
            || self.bottom.abs() > 100_000.0
        {
            return Err("Invalid anchor margins".into());
        }

        for percent in [self.width_percent, self.height_percent]
            .into_iter()
            .flatten()
        {
            if !percent.is_finite() || !(1.0..=200.0).contains(&percent) {
                return Err("Relative size must be between 1 and 200 percent".into());
            }
        }

        Ok(())
    }
}
