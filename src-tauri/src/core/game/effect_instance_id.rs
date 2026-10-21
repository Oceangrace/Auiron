use serde::Serialize;
use std::num::NonZeroU64;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub(crate) struct EffectInstanceId(NonZeroU64);

impl EffectInstanceId {
    pub(crate) fn new(value: u64) -> Result<Self, String> {
        let value = NonZeroU64::new(value).ok_or("EffectInstanceId must be positive")?;

        Ok(Self(value))
    }

    pub(crate) fn get(self) -> u64 {
        self.0.get()
    }
}
