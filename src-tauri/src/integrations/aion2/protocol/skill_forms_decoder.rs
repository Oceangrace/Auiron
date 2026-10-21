use super::BinaryReader;
use crate::core::game::GameEvent;
use crate::core::game::SkillForms;
use crate::core::game::SkillId;

/// 0x3900: the own character's skills with form slots and selected forms, sent on entering the world.
pub(super) struct SkillFormsDecoder;

impl SkillFormsDecoder {
    pub(super) fn decode(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let count = reader.byte()?;

        if !(1..=64).contains(&count) {
            return Err("Invalid skill form list count".into());
        }

        let skills = (0..count)
            .map(|_| Self::entry(reader))
            .collect::<Result<Vec<_>, _>>()?;

        if reader.remaining() != 0 {
            return Err("Unrecognized skill form list suffix".into());
        }

        Ok(vec![GameEvent::SkillFormsListed { skills }])
    }

    fn entry(reader: &mut BinaryReader<'_>) -> Result<SkillForms, String> {
        let skill_id = SkillId::new(reader.u32()?)?;

        let slots = reader.byte()?;

        if slots > 8 {
            return Err("Invalid skill form slot count".into());
        }

        let mut forms = Vec::new();

        for expected in 1..=slots {
            if reader.byte()? != expected {
                return Err("Unexpected skill form slot".into());
            }

            let form = reader.u32()?;

            if form != 0 {
                forms.push(SkillId::new(form)?);
            }
        }

        Ok(SkillForms::new(skill_id, forms))
    }
}
