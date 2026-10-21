use crate::core::game::ObservedEffect;

pub(super) struct EffectListCandidate {
    position: usize,
    entries: Vec<ObservedEffect>,
}

impl EffectListCandidate {
    pub(super) fn new(position: usize, entries: Vec<ObservedEffect>) -> Self {
        Self { position, entries }
    }

    pub(super) fn position(&self) -> usize {
        self.position
    }

    pub(super) fn entries(&self) -> &[ObservedEffect] {
        &self.entries
    }

    pub(super) fn into_entries(self) -> Vec<ObservedEffect> {
        self.entries
    }
}
