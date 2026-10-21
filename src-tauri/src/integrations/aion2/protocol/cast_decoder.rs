use super::BinaryReader;
use crate::core::game::CastPreparation;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::SkillId;

pub(super) struct CastDecoder;

impl CastDecoder {
    pub(super) fn decode(
        reader: &mut BinaryReader<'_>,
        at_us: u64,
    ) -> Result<Vec<GameEvent>, String> {
        let source_id = EntityId::new(reader.varint()?)?;

        let shape = reader.byte()?;

        if !matches!(shape, 0 | 2) {
            return Err("Unknown cast preparation shape".into());
        }

        let skill_id = SkillId::new(reader.u32()?)?;

        if reader.varint()? != 1 || reader.byte()? != 2 {
            return Err("Unknown cast preparation header".into());
        }

        let target = reader.varint()?;

        let target_id = if target >= 100 {
            Some(EntityId::new(target)?)
        } else {
            None
        };

        for _ in 0..4 {
            if !f32::from_bits(reader.u32()?).is_finite() {
                return Err("Invalid cast preparation coordinates".into());
            }
        }

        let duration_ms = u32::try_from(reader.varint()?).map_err(|_| "Invalid cast duration")?;

        // The ending byte is 1 or 2; the meaning of 2 is not identified.
        if !(1..=120_000).contains(&duration_ms) || !matches!(reader.byte()?, 1 | 2) {
            return Err("Invalid cast preparation duration or ending".into());
        }

        // Shape 2 appends the caster power scalar; summons inherit their owner's value.
        if shape == 2 {
            reader.varint()?;
        }

        if reader.remaining() != 0 || source_id.get() < 100 {
            return Err("Unknown cast preparation suffix or source".into());
        }

        Ok(vec![GameEvent::CastStarted {
            source_id,
            preparation: CastPreparation::new(skill_id, target_id, at_us, duration_ms),
        }])
    }
}
