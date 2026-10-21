use super::BinaryReader;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::SkillId;

pub(super) struct HitNoticeDecoder;

impl HitNoticeDecoder {
    pub(super) fn decode(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let source_id = EntityId::new(reader.varint()?)?;

        if reader.byte()? != 0 {
            return Err("Unknown hit notice header".into());
        }

        let entity_id = EntityId::new(reader.varint()?)?;

        let skill_id = SkillId::new(reader.u32()?)?;

        // Some skills omit the leading sequence byte; both forms must end exactly.
        if !Self::suffix(reader.fork(), true) && !Self::suffix(reader.fork(), false) {
            return Err("Invalid hit notice suffix".into());
        }

        if entity_id == source_id {
            return Ok(Vec::new());
        }

        Ok(vec![GameEvent::HitLanded {
            entity_id,
            source_id,
            skill_id,
        }])
    }

    fn suffix(mut reader: BinaryReader<'_>, sequence: bool) -> bool {
        Self::read_suffix(&mut reader, sequence).is_ok() && reader.remaining() == 0
    }

    fn read_suffix(reader: &mut BinaryReader<'_>, sequence: bool) -> Result<(), String> {
        if sequence {
            reader.byte()?;
        }

        reader.skip(4)?;

        for _ in 0..3 {
            if !f32::from_bits(reader.u32()?).is_finite() {
                return Err("Invalid hit notice position".into());
            }
        }

        reader.varint()?;
        reader.skip(6)
    }
}
