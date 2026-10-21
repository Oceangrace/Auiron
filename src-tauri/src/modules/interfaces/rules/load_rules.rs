use crate::integrations::aion2::GameId;
use crate::modules::interfaces::rules::LoadState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub(crate) struct LoadRules {
    #[schemars(length(max = 512))]
    pub(in crate::modules::interfaces) map_ids: Vec<GameId>,
    #[schemars(length(max = 512))]
    pub(in crate::modules::interfaces) dungeon_ids: Vec<GameId>,
    #[schemars(length(max = 512))]
    pub(in crate::modules::interfaces) npc_ids: Vec<GameId>,
    pub(in crate::modules::interfaces) in_combat: LoadState,
    pub(in crate::modules::interfaces) in_group: LoadState,
    #[schemars(length(max = 128))]
    pub(in crate::modules::interfaces) player_class: String,
    #[schemars(length(max = 256))]
    pub(in crate::modules::interfaces) zone: String,
}

impl Default for LoadRules {
    fn default() -> Self {
        Self {
            map_ids: Vec::new(),
            dungeon_ids: Vec::new(),
            npc_ids: Vec::new(),
            in_combat: LoadState::Any,
            in_group: LoadState::Any,
            player_class: String::new(),
            zone: String::new(),
        }
    }
}
