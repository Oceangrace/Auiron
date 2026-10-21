use super::ObservedEffect;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EffectSnapshot {
    disposition: Option<super::EffectDisposition>,
    display: Option<super::EffectDisplay>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    healing_rows: Vec<u8>,
    effect: ObservedEffect,
    applied_at_us: u64,
    expires_at_us: Option<u64>,
}

impl EffectSnapshot {
    pub(crate) fn new(
        effect: ObservedEffect,
        applied_at_us: u64,
        expires_at_us: Option<u64>,
    ) -> Self {
        Self {
            disposition: None,
            display: None,
            healing_rows: Vec::new(),
            effect,
            applied_at_us,
            expires_at_us,
        }
    }

    pub(crate) fn with_disposition(
        mut self,
        disposition: Option<super::EffectDisposition>,
    ) -> Self {
        self.disposition = disposition;
        self
    }

    pub(crate) fn disposition(&self) -> Option<super::EffectDisposition> {
        self.disposition
    }

    pub(crate) fn with_display(mut self, display: Option<super::EffectDisplay>) -> Self {
        self.display = display;

        self
    }

    pub(crate) fn display(&self) -> Option<&super::EffectDisplay> {
        self.display.as_ref()
    }

    pub(crate) fn with_healing_rows(mut self, rows: &[u8]) -> Self {
        self.healing_rows = rows.to_vec();

        self
    }

    pub(crate) fn is_healing_row(&self, row: u8) -> bool {
        self.healing_rows.contains(&row)
    }

    pub(crate) fn effect(&self) -> &ObservedEffect {
        &self.effect
    }

    pub(crate) fn applied_at_us(&self) -> u64 {
        self.applied_at_us
    }

    pub(crate) fn expires_at_us(&self) -> Option<u64> {
        self.expires_at_us
    }
}
