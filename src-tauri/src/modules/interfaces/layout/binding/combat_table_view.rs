use super::CombatDirection;
use super::CombatMetric;
use super::CombatScope;
use super::TableDimension;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CombatTableView {
    #[schemars(length(min = 1, max = 100))]
    pub(in crate::modules::interfaces) id: String,
    #[schemars(length(min = 1, max = 128))]
    pub(in crate::modules::interfaces) name: String,
    pub(in crate::modules::interfaces) metric: CombatMetric,
    pub(in crate::modules::interfaces) direction: CombatDirection,
    pub(in crate::modules::interfaces) dimension: TableDimension,
    pub(in crate::modules::interfaces) scope: CombatScope,
    pub(in crate::modules::interfaces) actor_id: Option<String>,
    pub(in crate::modules::interfaces) target_id: Option<String>,
}
