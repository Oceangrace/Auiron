use serde::{Deserialize, Serialize};

#[derive(
    Deserialize, Clone, Copy, Debug, Serialize, Eq, PartialEq, Ord, PartialOrd, schemars::JsonSchema,
)]
pub(crate) enum CharacterClass {
    Gladiator,
    Templar,
    Ranger,
    Assassin,
    Elementalist,
    Sorcerer,
    Cleric,
    Chanter,
}
