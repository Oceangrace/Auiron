use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct EncounterSchedule {
    pub(in crate::modules::interfaces) boss_only: bool,
    #[schemars(range(min = 0, max = 3_600_000))]
    pub(in crate::modules::interfaces) first_ms: f64,
    #[schemars(range(min = 0, max = 600_000))]
    pub(in crate::modules::interfaces) interval_ms: f64,
}
