use super::BinaryReader;
use super::damage_hit_type::DamageHitType;
use super::damage_record::DamageRecord;
use crate::core::game::EffectId;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::SkillId;
use crate::integrations::aion2::catalog::RuntimeCatalog;

pub(super) struct DamageDecoder;

impl DamageDecoder {
    pub(super) fn decode(
        reader: &mut BinaryReader<'_>,
        catalog: &RuntimeCatalog,
    ) -> Result<Vec<GameEvent>, String> {
        let start = reader.position();

        let events = match DamageRecord::read(reader) {
            Ok(record) => Self::events(record, catalog)?,
            Err(_) => {
                reader.seek(start)?;

                vec![Self::unknown(reader)?]
            }
        };

        Ok(events
            .into_iter()
            .filter(|event| {
                !event.skill_id().is_some_and(|id| {
                    catalog
                        .skill(id.get())
                        .is_some_and(|skill| skill.mana_only())
                })
            })
            .collect())
    }

    fn events(record: DamageRecord, catalog: &RuntimeCatalog) -> Result<Vec<GameEvent>, String> {
        let entity_id = record.recipient();

        let source_id = record.source();

        // The reflection notice only links the reflected hit; 0x3805 carries the counted damage.
        let Some(skill_id) = record.skill() else {
            return Ok(Vec::new());
        };

        let hit_type = record.hit_type();

        let effect_ids = record
            .effects()
            .iter()
            .map(|id| EffectId::new(*id))
            .collect::<Result<Vec<_>, _>>()?;

        let Some(amount) = record.into_amount() else {
            return Ok(vec![match hit_type {
                DamageHitType::Miss => GameEvent::Miss {
                    entity_id,
                    source_id,
                    skill_id,
                },
                DamageHitType::Resist => GameEvent::Resist {
                    entity_id,
                    source_id,
                    skill_id,
                    effect_ids,
                },
                _ => GameEvent::NonValueHit {
                    entity_id,
                    source_id,
                    skill_id,
                    result_code: hit_type.code(),
                },
            }]);
        };

        let value = amount.value();

        let hit = amount.hit(hit_type);

        Ok(vec![if entity_id == source_id {
            GameEvent::SelfRestoration {
                entity_id,
                skill_id,
                value,
                hit,
            }
        } else if catalog
            .skill(skill_id.get())
            .is_some_and(|skill| skill.healing_only())
        {
            GameEvent::Healing {
                entity_id,
                source_id,
                skill_id,
                value,
                hit: Some(hit),
            }
        } else {
            GameEvent::Damage {
                entity_id,
                source_id,
                skill_id,
                value,
                hit,
            }
        }])
    }

    fn unknown(reader: &mut BinaryReader<'_>) -> Result<GameEvent, String> {
        let entity_id = EntityId::new(reader.varint()?)?;

        reader.varint()?;
        reader.varint()?;

        let source_id = EntityId::new(reader.varint()?)?;

        let skill_id = reader.u32().ok().and_then(|raw| SkillId::new(raw).ok());

        Ok(GameEvent::DamageUnknown {
            entity_id,
            source_id,
            skill_id,
        })
    }
}
