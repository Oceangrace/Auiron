use super::BinaryReader;
use crate::core::game::GameEvent;
use crate::core::game::LearnedSkill;
use crate::core::game::SkillId;

/// 0x5100: the own character's learned skills and levels, sent on entering the world.
pub(super) struct SkillLevelsDecoder;

impl SkillLevelsDecoder {
    const PRESENT: u8 = 0x01;
    const SUFFIX: u8 = 0x02;
    const EXTRA: u8 = 0x04;

    pub(super) fn decode(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let count = reader.varint()?;

        if !(1..=1024).contains(&count) {
            return Err("Invalid skill list count".into());
        }

        let skills = (0..count)
            .map(|_| Self::entry(reader))
            .collect::<Result<Vec<_>, _>>()?;

        if reader.remaining() != 0 {
            return Err("Unrecognized skill list suffix".into());
        }

        Ok(vec![GameEvent::SkillsLearned { skills }])
    }

    fn entry(reader: &mut BinaryReader<'_>) -> Result<LearnedSkill, String> {
        let marker = reader.byte()?;

        if marker & Self::PRESENT == 0
            || marker & !(Self::PRESENT | Self::SUFFIX | Self::EXTRA) != 0
        {
            return Err("Unknown skill list entry marker".into());
        }

        let skill_id = SkillId::new(reader.u32()?)?;

        let level = reader.byte()?;

        let base_level = reader.byte()?;

        // Bonus byte and an opaque eight-byte block; the block changes while a skill cools down.
        reader.skip(9)?;

        if marker & Self::SUFFIX != 0 {
            reader.skip(2)?;
        }

        if marker & Self::EXTRA != 0 {
            reader.skip(1)?;
        }

        Ok(LearnedSkill::new(skill_id, level, base_level))
    }
}
