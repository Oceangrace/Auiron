use crate::core::game::CharacterClass;

pub(super) struct ClassDecoder;

impl ClassDecoder {
    pub(super) fn profile(code: u32) -> Option<CharacterClass> {
        if !(5..=36).contains(&code) {
            return None;
        }

        Some(match (code - 5) / 4 {
            0 => CharacterClass::Gladiator,
            1 => CharacterClass::Templar,
            2 => CharacterClass::Ranger,
            3 => CharacterClass::Assassin,
            4 => CharacterClass::Elementalist,
            5 => CharacterClass::Sorcerer,
            6 => CharacterClass::Cleric,
            7 => CharacterClass::Chanter,
            _ => return None,
        })
    }
}
