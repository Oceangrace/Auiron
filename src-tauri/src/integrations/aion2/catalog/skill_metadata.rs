use crate::core::game::CharacterClass;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SkillMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    class_name: Option<CharacterClass>,
    #[serde(default)]
    automatic: bool,
    cooldown_group: Option<NonZeroU32>,
    cooldown_ms: u32,
    casting_ms: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    charge_ms: Option<NonZeroU32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    charge_skill: Option<NonZeroU32>,
    mana_only: bool,
    #[serde(default)]
    healing_only: bool,
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    chain_previous: Option<NonZeroU32>,
    #[serde(default)]
    chain_window_ms: u32,
    #[serde(default)]
    requires_shock: bool,
}

impl SkillMetadata {
    pub(crate) fn class_name(&self) -> Option<CharacterClass> {
        self.class_name
    }

    pub(crate) fn automatic(&self) -> bool {
        self.automatic
    }

    pub(crate) fn requires_shock(&self) -> bool {
        self.requires_shock
    }

    pub(crate) fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub(crate) fn chain_previous(&self) -> Option<u32> {
        self.chain_previous.map(NonZeroU32::get)
    }

    pub(crate) fn chain_window_ms(&self) -> u32 {
        self.chain_window_ms
    }

    pub(crate) fn cooldown_group(&self) -> Option<u32> {
        self.cooldown_group.map(NonZeroU32::get)
    }
    pub(crate) fn cooldown_ms(&self) -> u32 {
        self.cooldown_ms
    }
    pub(crate) fn casting_ms(&self) -> u32 {
        self.casting_ms
    }

    pub(crate) fn charge_ms(&self) -> Option<u32> {
        self.charge_ms.map(NonZeroU32::get)
    }

    pub(crate) fn charge_skill(&self) -> Option<u32> {
        self.charge_skill.map(NonZeroU32::get)
    }
    pub(crate) fn healing_only(&self) -> bool {
        self.healing_only
    }

    pub(crate) fn mana_only(&self) -> bool {
        self.mana_only
    }
}
