use super::BinaryReader;
use super::DecoderKind;
use crate::core::game::EffectId;
use crate::core::game::EffectInstanceId;
use crate::core::game::EffectTiming;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::ObservedEffect;
use crate::core::game::SkillId;

pub(super) struct EffectDecoder;

impl EffectDecoder {
    pub(super) fn decode(
        reader: &mut BinaryReader<'_>,
        kind: DecoderKind,
    ) -> Result<Vec<GameEvent>, String> {
        let entity_id = EntityId::new(reader.varint()?)?;

        if kind == DecoderKind::EffectRemoved {
            return Self::removals(reader, entity_id);
        }

        let count = if kind == DecoderKind::EffectAdded {
            reader.byte()?
        } else {
            1
        };

        let mut events = Vec::new();

        for _ in 0..count {
            let effect = Self::entry(reader, false)?;

            let restarted = reader.byte()? == 2;

            reader.skip(12)?;
            events.push(match kind {
                DecoderKind::EffectAdded => GameEvent::EffectApplied {
                    entity_id,
                    effect,
                    restarted,
                },
                DecoderKind::EffectChanged => GameEvent::EffectUpdated {
                    entity_id,
                    effect,
                    restarted,
                },
                _ => return Err("Unexpected effect decoder".into()),
            });
        }

        Self::finish(reader)?;

        Ok(events)
    }

    pub(super) fn entry(
        reader: &mut BinaryReader<'_>,
        listed: bool,
    ) -> Result<ObservedEffect, String> {
        let flags = reader.byte()?;

        let instance_id = EffectInstanceId::new(reader.varint()?)?;

        let effect_id = EffectId::new(reader.u32()?)?;

        let duration = reader.i64()?;

        let end = reader.server_time()?;

        if duration < -1 {
            return Err("Invalid effect duration".into());
        }

        let source = if listed && flags & 1 == 0 {
            0
        } else {
            reader.varint()?
        };

        let source_id = if source == 0 {
            None
        } else {
            Some(EntityId::new(source)?)
        };

        let level = reader.byte()?;

        let skill_id = if flags & 2 != 0 {
            Some(SkillId::new(reader.u32()?)?)
        } else {
            None
        };

        let timing = if duration < 0 {
            EffectTiming::Untimed
        } else {
            EffectTiming::Timed {
                duration_ms: duration as u64,
                ends_at_server_ms: end,
            }
        };

        Ok(ObservedEffect::new(
            instance_id,
            effect_id,
            source_id,
            level,
            skill_id,
            timing,
        ))
    }

    fn removals(
        reader: &mut BinaryReader<'_>,
        entity_id: EntityId,
    ) -> Result<Vec<GameEvent>, String> {
        let count = reader.byte()?;

        let mut events = Vec::new();

        for _ in 0..count {
            let kind = reader.byte()?;

            let instance_id = EffectInstanceId::new(reader.varint()?)?;

            let reason = reader.byte()?;

            let mut removal_skill = None;

            let removed_by = if kind & 7 == 7 {
                let raw = reader.varint()?;

                let raw_skill = reader.u32()?;

                reader.skip(4)?;
                removal_skill = if raw_skill == 0 {
                    None
                } else {
                    Some(SkillId::new(raw_skill)?)
                };

                if raw == 0 {
                    None
                } else {
                    Some(EntityId::new(raw)?)
                }
            } else {
                None
            };

            events.push(GameEvent::EffectRemoved {
                entity_id,
                instance_id,
                reason,
                removed_by,
                removal_skill,
                effect: None,
            });
        }

        Self::finish(reader)?;

        Ok(events)
    }

    fn finish(reader: &BinaryReader<'_>) -> Result<(), String> {
        if reader.remaining() != 0 {
            return Err("Invalid effect message boundary".into());
        }

        Ok(())
    }
}
