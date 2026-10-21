use super::BinaryReader;
use super::DecoderKind;
use super::FrameBuffer;
use super::OpcodeTable;
use super::decode_budget::DecodeBudget;
use super::effect_decoder::EffectDecoder;
use super::effect_list_decoder::EffectListDecoder;
use super::identity_decoder::IdentityDecoder;
use super::simple_decoder::SimpleDecoder;
use super::spawn_decoder::SpawnDecoder;
use crate::core::game::GameEvent;
use crate::core::game::PartyRoster;
use crate::integrations::aion2::catalog::NpcIndex;
use crate::integrations::aion2::catalog::RuntimeCatalog;
use std::collections::BTreeMap;
use std::sync::Arc;

pub(crate) struct MessageDecoder {
    table: OpcodeTable,
    npcs: NpcIndex,
    catalog: Arc<RuntimeCatalog>,
    messages: BTreeMap<u16, u64>,
    rejected: BTreeMap<u16, u64>,
    unknown: BTreeMap<u16, u64>,
    unsupported: BTreeMap<u16, u64>,
}

impl MessageDecoder {
    pub(crate) fn new(table: OpcodeTable, npcs: NpcIndex, catalog: Arc<RuntimeCatalog>) -> Self {
        Self {
            table,
            npcs,
            catalog,
            messages: BTreeMap::new(),
            rejected: BTreeMap::new(),
            unknown: BTreeMap::new(),
            unsupported: BTreeMap::new(),
        }
    }

    pub(crate) fn messages(&self) -> &BTreeMap<u16, u64> {
        &self.messages
    }

    pub(crate) fn rejected(&self) -> &BTreeMap<u16, u64> {
        &self.rejected
    }

    pub(crate) fn unknown(&self) -> &BTreeMap<u16, u64> {
        &self.unknown
    }

    pub(crate) fn unsupported(&self) -> &BTreeMap<u16, u64> {
        &self.unsupported
    }

    pub(crate) fn decode(
        &mut self,
        body: &[u8],
        budget: &mut DecodeBudget,
        at_us: u64,
    ) -> Result<Vec<GameEvent>, String> {
        self.decode_level(body, 0, budget, at_us)
    }

    fn decode_level(
        &mut self,
        body: &[u8],
        depth: u8,
        budget: &mut DecodeBudget,
        at_us: u64,
    ) -> Result<Vec<GameEvent>, String> {
        budget.message(body.len())?;

        if depth > 4 || body.len() < 2 {
            return Err("Protocol nesting limit or truncated opcode".into());
        }

        let mut reader = BinaryReader::new(body);

        let opcode = reader.u16()?;

        let kind = self.table.decoder(opcode);

        *self.messages.entry(opcode).or_default() += 1;

        if kind == Some(DecoderKind::Bundle) {
            return self.bundle(&mut reader, depth, budget, at_us);
        }

        let Some(kind) = kind else {
            let counts = if self.table.contains(opcode) {
                &mut self.unsupported
            } else {
                &mut self.unknown
            };

            *counts.entry(opcode).or_default() += 1;

            return Ok(Vec::new());
        };

        match self.message(body, &mut reader, kind, at_us) {
            Ok(events) => Ok(events),
            Err(_) => {
                *self.rejected.entry(opcode).or_default() += 1;

                Ok(Vec::new())
            }
        }
    }

    fn bundle(
        &mut self,
        reader: &mut BinaryReader<'_>,
        depth: u8,
        budget: &mut DecodeBudget,
        at_us: u64,
    ) -> Result<Vec<GameEvent>, String> {
        let expected = reader.u32()? as usize;

        if !(2..=FrameBuffer::MAX_BODY).contains(&expected) {
            return Err("Invalid LZ4 expansion size".into());
        }

        budget.expansion(expected)?;

        let mut output = vec![0; expected];

        let written =
            lz4_flex::block::decompress_into(reader.bytes(reader.remaining())?, &mut output)
                .map_err(|error| format!("Invalid LZ4 block: {error}"))?;

        if written != expected {
            return Err("LZ4 expansion size mismatch".into());
        }

        let mut framer = FrameBuffer::new();

        let frames = framer.push(&output)?;

        framer.finish()?;

        let mut events = Vec::new();

        for frame in frames {
            events.extend(self.decode_level(&frame, depth + 1, budget, at_us)?);
        }

        Ok(events)
    }

    fn message(
        &self,
        body: &[u8],
        reader: &mut BinaryReader<'_>,
        kind: DecoderKind,
        at_us: u64,
    ) -> Result<Vec<GameEvent>, String> {
        match kind {
            DecoderKind::CastStarted => super::cast_decoder::CastDecoder::decode(reader, at_us),
            DecoderKind::ChargeStarted => {
                super::charge_decoder::ChargeDecoder::decode(reader, at_us, &self.catalog)
            }
            DecoderKind::HealthPair => {
                super::health_pair_decoder::HealthPairDecoder::decode(reader)
            }

            DecoderKind::HitNotice => super::hit_notice_decoder::HitNoticeDecoder::decode(reader),
            DecoderKind::InstanceTimer => {
                super::instance_timer_decoder::InstanceTimerDecoder::decode(reader)
            }

            DecoderKind::OdyleList => super::odyle_decoder::OdyleDecoder::list(reader),
            DecoderKind::OdyleUpdate => super::odyle_decoder::OdyleDecoder::update(reader),
            DecoderKind::WorldBossList => {
                super::world_boss_decoder::WorldBossDecoder::decode(reader, at_us)
            }

            DecoderKind::ServerBattle => {
                super::battle_decoder::BattleDecoder::decode(reader, at_us)
            }

            DecoderKind::Ownership => {
                super::attribution_decoder::AttributionDecoder::decode(reader)
            }

            DecoderKind::EntityProperties => {
                super::property_decoder::PropertyDecoder::decode(reader)
            }

            DecoderKind::SkillConfirmation => {
                super::cooldown_decoder::CooldownDecoder::confirmation(reader)
            }

            DecoderKind::CooldownRemaining => {
                super::cooldown_decoder::CooldownDecoder::remaining(reader)
            }

            DecoderKind::PartyLeaveWorld | DecoderKind::PartyLeaveInstance => {
                if reader.bytes(2)? != [0, 0] || reader.remaining() != 0 {
                    return Err("Unknown party leave shape".into());
                }

                Ok(vec![GameEvent::Party {
                    roster: PartyRoster::new(Vec::new(), true, None),
                }])
            }

            DecoderKind::TargetSelected => super::target_decoder::TargetDecoder::decode(reader),
            DecoderKind::NpcDialogOpened => {
                super::npc_dialog_decoder::NpcDialogDecoder::opened(reader)
            }

            DecoderKind::NpcDialogClosed => {
                super::npc_dialog_decoder::NpcDialogDecoder::closed(reader)
            }

            DecoderKind::Party => super::party_decoder::PartyDecoder::decode(body),
            DecoderKind::SkillLevels => {
                super::skill_levels_decoder::SkillLevelsDecoder::decode(reader)
            }

            DecoderKind::SkillLevel => {
                super::skill_level_decoder::SkillLevelDecoder::decode(reader)
            }

            DecoderKind::SkillForms => {
                super::skill_forms_decoder::SkillFormsDecoder::decode(reader)
            }

            DecoderKind::PeriodicDamage => {
                super::periodic_decoder::PeriodicDecoder::decode(reader, &self.catalog)
            }

            DecoderKind::DirectDamage => {
                super::damage_decoder::DamageDecoder::decode(reader, &self.catalog)
            }

            DecoderKind::Spawn | DecoderKind::EntitySpawn => {
                let mut events = vec![SpawnDecoder::decode(body, &self.npcs)?];

                if kind == DecoderKind::Spawn {
                    events.extend(EffectListDecoder::decode(body, &[7, 2])?);
                }

                Ok(events)
            }

            DecoderKind::EffectAdded | DecoderKind::EffectChanged | DecoderKind::EffectRemoved => {
                EffectDecoder::decode(reader, kind)
            }

            DecoderKind::Identity => IdentityDecoder::decode(body, kind).map(|event| vec![event]),
            DecoderKind::SelfIdentity | DecoderKind::PlayerSpawn => {
                let mut events = IdentityDecoder::scan(body, &self.table);

                if kind == DecoderKind::PlayerSpawn
                    && let Ok(Some(profile)) =
                        super::player_profile_decoder::PlayerProfileDecoder::decode(body)
                {
                    events.push(profile);
                }

                if kind == DecoderKind::SelfIdentity
                    && let Ok(Some(vitals)) =
                        super::self_vitals_decoder::SelfVitalsDecoder::decode(body)
                {
                    events.push(vitals);
                }

                if kind == DecoderKind::PlayerSpawn
                    && let Ok(Some(vitals)) =
                        super::player_vitals_decoder::PlayerVitalsDecoder::decode(body)
                {
                    events.push(vitals);
                }

                events.extend(EffectListDecoder::decode(body, &[15])?);

                Ok(events)
            }

            _ => SimpleDecoder::decode(body, kind),
        }
    }
}
