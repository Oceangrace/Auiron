use super::EncounterSchedule;
use super::EventParticipant;
use super::GameEventType;
use crate::integrations::aion2::GameId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct EventTrigger {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::modules::interfaces) schedule: Option<EncounterSchedule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::modules::interfaces) source_npc_id: Option<GameId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(length(min = 1, max = 256))]
    pub(in crate::modules::interfaces) source_actor_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::modules::interfaces) use_widget_skill: Option<bool>,
    pub(in crate::modules::interfaces) event: GameEventType,
    pub(in crate::modules::interfaces) source: EventParticipant,
    pub(in crate::modules::interfaces) recipient: EventParticipant,
    pub(in crate::modules::interfaces) skill_id: Option<GameId>,
    pub(in crate::modules::interfaces) effect_id: Option<GameId>,
    #[schemars(range(min = 0, max = 1e12))]
    pub(in crate::modules::interfaces) min_value: Option<f64>,
    #[schemars(range(min = 100, max = 3_600_000))]
    pub(in crate::modules::interfaces) duration_ms: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 0, max = 3_600_000))]
    pub(in crate::modules::interfaces) delay_ms: Option<f64>,
}
