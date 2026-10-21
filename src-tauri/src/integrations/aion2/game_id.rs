use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(Clone, Copy, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(transparent)]
pub(crate) struct GameId {
    #[schemars(range(min = 1, max = u32::MAX))]
    value: NonZeroU32,
}

impl GameId {
    pub(crate) fn new(value: u32) -> Result<Self, String> {
        Ok(Self {
            value: NonZeroU32::new(value).ok_or("Game ID must be positive")?,
        })
    }

    pub(crate) fn get(self) -> u32 {
        self.value.get()
    }
}
