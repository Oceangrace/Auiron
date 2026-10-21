use super::BinaryReader;
use super::DecoderKind;
use super::identity_decoder::IdentityDecoder;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::MapId;
use crate::core::game::SkillId;
use std::collections::BTreeMap;

pub(super) struct SimpleDecoder;

impl SimpleDecoder {
    pub(super) fn decode(body: &[u8], kind: DecoderKind) -> Result<Vec<GameEvent>, String> {
        let mut reader = BinaryReader::new(body);

        reader.skip(2)?;

        match kind {
            DecoderKind::Clock => {
                let server_ms =
                    u64::try_from(reader.i64()?).map_err(|_| "Negative server clock")?;

                Ok(vec![GameEvent::Clock { server_ms }])
            }

            DecoderKind::MapLoad => {
                reader.skip(4)?;

                Ok(vec![GameEvent::MapChanged {
                    map_id: MapId::new(reader.u32()?)?,
                }])
            }

            DecoderKind::SkillUse => Self::skill(&mut reader),
            DecoderKind::Life => Self::life(&mut reader),
            DecoderKind::ComboAvailable | DecoderKind::ComboUnavailable => {
                Self::combo(&mut reader, kind)
            }

            DecoderKind::AllStats | DecoderKind::StatsChanged => {
                Self::stats(body, &mut reader, kind)
            }

            _ => Err("Unexpected simple message decoder".into()),
        }
    }

    fn skill(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let source = reader.varint()?;

        reader.skip(1)?;

        let skill = reader.u32()?;

        // A u16 use counter. Read as a varint, counters from 0x80 shifted the target by a byte.
        reader.skip(2)?;

        let entity = reader.varint()?;

        if source < 100 || skill == 0 {
            return Ok(Vec::new());
        }

        Ok(vec![GameEvent::SkillUsed {
            entity_id: if entity >= 100 {
                Some(EntityId::new(entity)?)
            } else {
                None
            },
            source_id: EntityId::new(source)?,
            skill_id: SkillId::new(skill)?,
        }])
    }

    fn life(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let entity_id = EntityId::new(reader.varint()?)?;

        reader.varint()?;

        Ok(match reader.varint()? {
            3 => vec![GameEvent::Died { entity_id }],
            7 => vec![GameEvent::Despawned { entity_id }],
            _ => Vec::new(),
        })
    }

    fn combo(reader: &mut BinaryReader<'_>, kind: DecoderKind) -> Result<Vec<GameEvent>, String> {
        let previous = if kind == DecoderKind::ComboAvailable {
            Some(SkillId::new(reader.u32()?)?)
        } else {
            None
        };

        if reader.byte()? != 1 {
            return Ok(Vec::new());
        }

        let raw = reader.u32()?;

        if raw == 0 || reader.remaining() != 0 {
            return Ok(Vec::new());
        }

        let skill_id = SkillId::new(raw)?;

        Ok(vec![match previous {
            Some(previous_skill_id) => GameEvent::ComboAvailable {
                previous_skill_id,
                skill_id,
            },
            None => GameEvent::ComboUnavailable { skill_id },
        }])
    }

    fn stats(
        body: &[u8],
        reader: &mut BinaryReader<'_>,
        kind: DecoderKind,
    ) -> Result<Vec<GameEvent>, String> {
        let entity_id = if kind == DecoderKind::StatsChanged {
            Some(EntityId::new(reader.varint()?)?)
        } else {
            reader.skip(2)?;
            None
        };

        let count = reader.byte()?;

        let mut values = BTreeMap::new();

        for _ in 0..count {
            values.insert(reader.u16()?, reader.i32()?);
        }

        if reader.remaining() != 8 {
            return if kind == DecoderKind::StatsChanged {
                IdentityDecoder::decode(body, kind).map(|event| vec![event])
            } else {
                Ok(Vec::new())
            };
        }

        let mut events = Vec::new();

        if let Some(entity_id) = entity_id {
            events.push(GameEvent::SelfIdentified {
                entity_id,
                authoritative: true,
            });
        }

        events.push(GameEvent::StatsChanged { entity_id, values });

        Ok(events)
    }
}
