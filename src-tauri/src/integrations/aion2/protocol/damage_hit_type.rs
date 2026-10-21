/// The hit type field of a 0x3804 record. Unlisted codes stay opaque.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DamageHitType {
    Miss,
    Critical,
    Resist,
    Reflection,
    Other(u64),
}

impl DamageHitType {
    pub(super) fn code(self) -> u64 {
        match self {
            Self::Miss => 1,
            Self::Critical => 3,
            Self::Resist => 6,
            Self::Reflection => 9,
            Self::Other(code) => code,
        }
    }
}

impl From<u64> for DamageHitType {
    fn from(code: u64) -> Self {
        match code {
            1 => Self::Miss,
            3 => Self::Critical,
            6 => Self::Resist,
            9 => Self::Reflection,
            _ => Self::Other(code),
        }
    }
}
