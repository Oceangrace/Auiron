use super::BinaryReader;
use crate::core::game::CastPreparation;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::SkillId;
use crate::integrations::aion2::catalog::RuntimeCatalog;

pub(super) struct ChargeDecoder;

impl ChargeDecoder {
    pub(super) fn decode(
        reader: &mut BinaryReader<'_>,
        at_us: u64,
        catalog: &RuntimeCatalog,
    ) -> Result<Vec<GameEvent>, String> {
        let source_id = EntityId::new(reader.varint()?)?;

        if source_id.get() < 100 || reader.byte()? != 1 {
            return Err("Unknown charge source or shape".into());
        }

        let skill_id = SkillId::new(reader.u32()?)?;

        reader.byte()?;

        if reader.byte()? != 2 {
            return Err("Unknown charge target shape".into());
        }

        let target = reader.varint()?;

        let target_id = if target >= 100 {
            Some(EntityId::new(target)?)
        } else {
            None
        };

        // Four position/orientation floats and an opaque scalar; none encodes duration.
        for _ in 0..5 {
            if !f32::from_bits(reader.u32()?).is_finite() {
                return Err("Invalid charge coordinates or scalar".into());
            }
        }

        if reader.byte()? != 1 || reader.remaining() != 0 {
            return Err("Unknown charge suffix".into());
        }

        let Some(duration_ms) = catalog
            .skill(skill_id.get())
            .and_then(|skill| skill.charge_ms())
        else {
            return Ok(Vec::new());
        };

        Ok(vec![GameEvent::CastStarted {
            source_id,
            preparation: CastPreparation::new(skill_id, target_id, at_us, duration_ms),
        }])
    }
}
