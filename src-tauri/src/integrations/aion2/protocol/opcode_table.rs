use super::DecoderKind;
use super::OpcodeDefinition;
use crate::core::game::GameCapabilities;
use crate::core::game::GameEventType;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::collections::HashSet;

#[derive(Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct OpcodeTable {
    #[schemars(range(min = 1, max = 1))]
    schema_version: u8,
    client_build: u32,
    messages: Vec<OpcodeDefinition>,
}

impl OpcodeTable {
    pub(crate) fn parse(bytes: &[u8], build: u32) -> Result<Self, String> {
        let table: Self = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;

        table.validate(build)?;

        Ok(table)
    }

    pub(crate) fn validate(&self, build: u32) -> Result<(), String> {
        if self.schema_version != 1 || self.client_build != build || self.messages.is_empty() {
            return Err("Unsupported protocol table version or build".into());
        }

        let mut numbers = HashSet::new();

        let mut names = HashSet::new();

        let mut decoders = HashSet::new();

        for message in &self.messages {
            message.validate()?;

            if !numbers.insert(message.opcode()) || !names.insert(message.name()) {
                return Err("Duplicate protocol opcode or name".into());
            }

            if let Some(decoder) = message.decoder()
                && !decoders.insert(decoder)
            {
                return Err("Duplicate protocol decoder assignment".into());
            }
        }

        if !decoders.contains(&DecoderKind::Bundle) {
            return Err("Protocol bundle decoder is required".into());
        }

        Ok(())
    }

    pub(crate) fn capabilities(&self) -> GameCapabilities {
        let kinds: HashSet<DecoderKind> = self
            .messages
            .iter()
            .filter_map(OpcodeDefinition::decoder)
            .collect();

        let mappings = [
            (DecoderKind::CastStarted, GameEventType::CastStarted),
            (DecoderKind::ChargeStarted, GameEventType::CastStarted),
            (DecoderKind::HitNotice, GameEventType::HitLanded),
            (
                DecoderKind::ServerBattle,
                GameEventType::ServerBattleChanged,
            ),
            (DecoderKind::PeriodicDamage, GameEventType::Damage),
            (DecoderKind::PeriodicDamage, GameEventType::Healing),
            (DecoderKind::DirectDamage, GameEventType::Damage),
            (DecoderKind::DirectDamage, GameEventType::Healing),
            (DecoderKind::DirectDamage, GameEventType::Miss),
            (DecoderKind::DirectDamage, GameEventType::Resist),
            (DecoderKind::SkillUse, GameEventType::SkillUsed),
            (DecoderKind::EffectAdded, GameEventType::EffectApplied),
            (DecoderKind::EffectChanged, GameEventType::EffectUpdated),
            (DecoderKind::EffectRemoved, GameEventType::EffectRemoved),
            (DecoderKind::MapLoad, GameEventType::MapChanged),
            (DecoderKind::Life, GameEventType::Death),
            (DecoderKind::Life, GameEventType::Despawn),
            (DecoderKind::ComboAvailable, GameEventType::ComboAvailable),
            (
                DecoderKind::ComboUnavailable,
                GameEventType::ComboUnavailable,
            ),
        ];

        let mut events: BTreeSet<GameEventType> = mappings
            .into_iter()
            .filter_map(|(decoder, event)| kinds.contains(&decoder).then_some(event))
            .collect();

        if kinds.contains(&DecoderKind::DirectDamage) {
            events.extend([
                GameEventType::CombatStarted,
                GameEventType::CombatEnded,
                GameEventType::EncounterStarted,
                GameEventType::EncounterEnded,
            ]);
        }

        events.insert(GameEventType::HealthChanged);

        let lists = [
            DecoderKind::SelfIdentity,
            DecoderKind::PlayerSpawn,
            DecoderKind::Spawn,
        ]
        .iter()
        .any(|kind| kinds.contains(kind));

        let effects = lists
            || [
                DecoderKind::EffectAdded,
                DecoderKind::EffectChanged,
                DecoderKind::EffectRemoved,
            ]
            .iter()
            .any(|kind| kinds.contains(kind));

        GameCapabilities::new(
            events,
            effects,
            kinds.contains(&DecoderKind::Party),
            kinds.contains(&DecoderKind::SelfIdentity),
            kinds.contains(&DecoderKind::Spawn) || kinds.contains(&DecoderKind::EntitySpawn),
            kinds.contains(&DecoderKind::MapLoad),
        )
    }

    pub(crate) fn decoder(&self, opcode: u16) -> Option<DecoderKind> {
        self.messages
            .iter()
            .find(|message| message.opcode() == opcode)?
            .decoder()
    }

    pub(crate) fn contains(&self, opcode: u16) -> bool {
        self.messages
            .iter()
            .any(|message| message.opcode() == opcode)
    }
}
