use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::NonZeroU64;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(into = "String")]
pub(crate) struct CharacterId(NonZeroU64);

impl CharacterId {
    pub(crate) fn new(value: u64) -> Result<Self, String> {
        let value = NonZeroU64::new(value).ok_or("Character ID must be positive")?;

        if !(1..=9999).contains(&(value.get() >> 48)) {
            return Err("Invalid character server ID".into());
        }

        Ok(Self(value))
    }

    pub(crate) fn server_id(self) -> u16 {
        (self.0.get() >> 48) as u16
    }
}

impl From<CharacterId> for String {
    fn from(value: CharacterId) -> Self {
        value.0.to_string()
    }
}

impl Display for CharacterId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
