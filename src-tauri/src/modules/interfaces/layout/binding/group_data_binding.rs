use super::CombatDirection;
use super::CombatMetric;
use super::CombatScope;
use super::CombatTableView;
use super::GroupDataSource;
use super::TableDimension;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct GroupDataBinding {
    // Retained for profiles saved by the previous fixed-alert implementation.
    #[serde(default = "GroupDataBinding::default_boss_alert_minutes")]
    #[schemars(length(max = 3))]
    pub(in crate::modules::interfaces) boss_alert_minutes: Vec<u32>,
    #[serde(default)]
    #[schemars(length(max = 48))]
    pub(in crate::modules::interfaces) boss_ids: Vec<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::modules::interfaces) effects: Option<super::GroupEffectSettings>,
    #[schemars(length(max = 16))]
    pub(in crate::modules::interfaces) views: Vec<CombatTableView>,
    pub(in crate::modules::interfaces) active_view_id: Option<String>,
    pub(in crate::modules::interfaces) show_breakdown: bool,
    pub(in crate::modules::interfaces) direction: CombatDirection,
    pub(in crate::modules::interfaces) dimension: TableDimension,
    pub(in crate::modules::interfaces) actor_id: Option<String>,
    pub(in crate::modules::interfaces) target_id: Option<String>,
    pub(in crate::modules::interfaces) include_summons: bool,
    pub(in crate::modules::interfaces) scope: CombatScope,
    pub(in crate::modules::interfaces) source: GroupDataSource,
    pub(in crate::modules::interfaces) metric: CombatMetric,
    #[schemars(range(min = 1, max = 128))]
    pub(in crate::modules::interfaces) limit: u32,
}

impl GroupDataBinding {
    fn default_boss_alert_minutes() -> Vec<u32> {
        vec![5, 3, 1]
    }
}
