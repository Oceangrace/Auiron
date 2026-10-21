use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

#[derive(Deserialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub(crate) struct EntityId(NonZeroU64);

impl EntityId {
    pub(crate) fn new(value: u64) -> Result<Self, String> {
        let value = NonZeroU64::new(value).ok_or("EntityId must be positive")?;

        Ok(Self(value))
    }

    pub(crate) fn get(self) -> u64 {
        self.0.get()
    }
}
