use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct OdyleEnergy {
    basic: u32,
    charged: u32,
}

impl OdyleEnergy {
    pub(crate) fn new(basic: u32, charged: u32) -> Self {
        Self { basic, charged }
    }

    pub(crate) fn basic(&self) -> u32 {
        self.basic
    }

    pub(crate) fn charged(&self) -> u32 {
        self.charged
    }
}
