use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct NpcMetadata {
    boss: bool,
    player_summon: bool,
}

impl NpcMetadata {
    pub(crate) fn boss(&self) -> bool {
        self.boss
    }
    pub(crate) fn player_summon(&self) -> bool {
        self.player_summon
    }
}
