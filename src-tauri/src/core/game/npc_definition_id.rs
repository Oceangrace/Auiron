use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(Deserialize, Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub(crate) struct NpcDefinitionId(NonZeroU32);

impl NpcDefinitionId {
    pub(crate) fn new(value: u32) -> Result<Self, String> {
        let value = NonZeroU32::new(value).ok_or("NPC definition ID must be positive")?;

        Ok(Self(value))
    }

    pub(crate) fn get(self) -> u32 {
        self.0.get()
    }
}
