use crate::core::game::CharacterClass;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::GameSnapshot;
use crate::core::game::HealthSnapshot;
use crate::core::game::ManaSnapshot;
use crate::core::game::SkillBook;
use serde::{Deserialize, Serialize};

/// Resources retain their last observed values, explicitly marked as restored until refreshed.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PlayerRecovery {
    entity_id: EntityId,
    name: Option<String>,
    class_name: Option<CharacterClass>,
    server_id: Option<u16>,
    level: Option<u32>,
    skills: SkillBook,
    #[serde(default)]
    health: HealthSnapshot,
    #[serde(default)]
    mana: ManaSnapshot,
}

impl PlayerRecovery {
    pub(crate) fn capture(game: &GameSnapshot) -> Option<Self> {
        let entity_id = game.player_id()?;

        let unit = game.entities().get(&entity_id)?;

        if !game.player_in_world() {
            return None;
        }

        let identity = unit.identity();

        Some(Self {
            entity_id,
            name: identity.name().map(str::to_owned),
            class_name: identity.class_name(),
            server_id: identity.server_id(),
            level: identity.level(),
            skills: game.skill_book().clone(),
            health: unit.health().clone(),
            mana: unit.mana().clone(),
        })
    }

    pub(crate) fn identity(&self) -> GameEvent {
        GameEvent::Identity {
            entity_id: self.entity_id,
            name: self.name.clone(),
            own: true,
            server_id: self.server_id,
            class_name: self.class_name,
            level: self.level,
        }
    }

    pub(crate) fn skills(&self) -> &SkillBook {
        &self.skills
    }

    pub(crate) fn health(&self) -> &HealthSnapshot {
        &self.health
    }

    pub(crate) fn mana(&self) -> &ManaSnapshot {
        &self.mana
    }
}
