use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CatalogScope {
    Skills,
    Effects,
    Enemies,
    EnemySkills,
    EnemyEffects,
}
