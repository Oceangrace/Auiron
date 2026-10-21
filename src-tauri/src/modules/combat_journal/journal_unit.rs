use crate::core::game::CharacterClass;
use crate::core::game::EntityId;
use crate::core::game::GameSnapshot;
use crate::core::game::NpcDefinitionId;
use crate::core::game::UnitSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct JournalUnit {
    id: EntityId,
    owner_id: Option<EntityId>,
    name: Option<String>,
    class_name: Option<CharacterClass>,
    npc_id: Option<NpcDefinitionId>,
    player: bool,
    group: Option<bool>,
}

impl JournalUnit {
    pub(super) fn new(unit: &UnitSnapshot, snapshot: &GameSnapshot) -> Self {
        let member = snapshot.party().and_then(|party| {
            party
                .members()
                .iter()
                .find(|member| member.entity_id() == Some(unit.id()))
        });

        let class_name = unit
            .identity()
            .class_name()
            .or_else(|| member.and_then(|member| member.identity().class_name()));

        Self {
            id: unit.id(),
            owner_id: unit.owner_id(),
            name: unit.identity().name().map(str::to_owned),
            class_name,
            npc_id: unit.npc_id(),
            player: unit.npc_id().is_none()
                && (snapshot.player_id() == Some(unit.id())
                    || unit.identity().character_id().is_some()
                    || class_name.is_some()),
            group: snapshot.group_member(unit.id()),
        }
    }
}
