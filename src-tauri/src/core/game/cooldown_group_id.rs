use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub(crate) struct CooldownGroupId(u32);

impl CooldownGroupId {
    pub(crate) fn new(value: u32) -> Result<Self, String> {
        if value == 0 {
            return Err("Invalid cooldown group ID".into());
        }

        Ok(Self(value))
    }

    pub(crate) fn get(self) -> u32 {
        self.0
    }
}
