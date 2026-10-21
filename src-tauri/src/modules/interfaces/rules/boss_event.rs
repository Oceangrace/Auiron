use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum BossEvent {
    BeforeSpawn,
    Spawned,
    Alive,
    Unknown,
}
