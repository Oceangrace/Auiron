use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub(crate) struct InterfaceDisplay {
    /// Absent: positions are absolute screen pixels and only the manual scale applies.
    #[schemars(range(min = 320, max = 16384))]
    pub(in crate::modules::interfaces) base_width: Option<u32>,
    #[schemars(range(min = 240, max = 16384))]
    pub(in crate::modules::interfaces) base_height: Option<u32>,
    pub(in crate::modules::interfaces) auto_scale: bool,
    #[schemars(range(min = 25.0, max = 400.0))]
    pub(in crate::modules::interfaces) scale: f64,
}

impl Default for InterfaceDisplay {
    fn default() -> Self {
        Self {
            base_width: None,
            base_height: None,
            auto_scale: true,
            scale: 100.0,
        }
    }
}

impl InterfaceDisplay {
    pub(in crate::modules::interfaces) fn validate(&self) -> Result<(), String> {
        let base = match (self.base_width, self.base_height) {
            (Some(width), Some(height)) => {
                (320..=16384).contains(&width) && (240..=16384).contains(&height)
            }
            (None, None) => true,
            _ => false,
        };

        if !base || !self.scale.is_finite() || !(25.0..=400.0).contains(&self.scale) {
            return Err("Invalid HUD base resolution or scale".into());
        }

        Ok(())
    }
}
