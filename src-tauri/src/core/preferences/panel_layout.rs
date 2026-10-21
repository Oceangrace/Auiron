use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PanelLayout {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl PanelLayout {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if !(-100_000.0..=100_000.0).contains(&self.x)
            || !(-100_000.0..=100_000.0).contains(&self.y)
            || !(1.0..=16_384.0).contains(&self.width)
            || !(1.0..=16_384.0).contains(&self.height)
        {
            return Err("Invalid panel layout".into());
        }

        Ok(())
    }
}
