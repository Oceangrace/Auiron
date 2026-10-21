use super::BinaryReader;
use crate::core::game::CooldownGroupId;
use crate::core::game::GameEvent;
use crate::core::game::SkillId;

pub(super) struct CooldownDecoder;

impl CooldownDecoder {
    pub(super) fn remaining(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        if reader.byte()? != 1 {
            return Err("Unknown cooldown update shape".into());
        }

        let group_id = CooldownGroupId::new(reader.u32()?)?;

        let remaining_ms =
            u32::try_from(reader.varint()?).map_err(|_| "Invalid cooldown remainder")?;

        if reader.remaining() != 0 {
            return Err("Unknown cooldown update suffix".into());
        }

        Ok(vec![GameEvent::CooldownChanged {
            group_id,
            remaining_ms,
        }])
    }

    pub(super) fn confirmation(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        if reader.bytes(3)? != [0, 0, 0] {
            return Ok(Vec::new());
        }

        let skill_id = SkillId::new(reader.u32()?)?;

        Ok(vec![GameEvent::SkillConfirmed { skill_id }])
    }
}
