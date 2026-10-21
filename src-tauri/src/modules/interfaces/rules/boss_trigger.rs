use super::BossEvent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct BossTrigger {
    pub(in crate::modules::interfaces) boss_id: u32,
    pub(in crate::modules::interfaces) event: BossEvent,
    #[schemars(range(min = 0, max = 86400))]
    pub(in crate::modules::interfaces) before_seconds: f64,
    #[schemars(range(min = 100, max = 3_600_000))]
    pub(in crate::modules::interfaces) duration_ms: f64,
}
