use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct JournalPanelLayout {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl JournalPanelLayout {
    pub(super) fn validate(&self) -> Result<(), String> {
        // Allow a panel reduced to a small game viewport; retain its requested placement.
        if !(-100_000.0..=100_000.0).contains(&self.x)
            || !(-100_000.0..=100_000.0).contains(&self.y)
            || !(1.0..=16_384.0).contains(&self.width)
            || !(1.0..=16_384.0).contains(&self.height)
        {
            return Err("Invalid journal panel layout".into());
        }

        Ok(())
    }
}
