use super::BinaryReader;
use crate::core::game::GameEvent;
use crate::core::game::LearnedSkill;
use crate::core::game::SkillId;

/// 0x3813: one own skill with its current level, sent on login and after each level change.
pub(super) struct SkillLevelDecoder;

impl SkillLevelDecoder {
    pub(super) fn decode(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        if reader.byte()? != 1 {
            return Err("Unknown skill level count".into());
        }

        let key = reader.u32()?;

        if reader.byte()? != 1 || reader.u32()? != key {
            return Err("Unknown skill level entry".into());
        }

        let skill_id = SkillId::new(key)?;

        let level = reader.byte()?;

        let base_level = reader.byte()?;

        // Bonus byte, a four-byte block and a trailing code (10 on login, 3 or 5 after a change).
        reader.skip(6)?;

        if reader.remaining() != 0 {
            return Err("Unrecognized skill level suffix".into());
        }

        Ok(vec![GameEvent::SkillLevelChanged {
            skill: LearnedSkill::new(skill_id, level, base_level),
        }])
    }
}
