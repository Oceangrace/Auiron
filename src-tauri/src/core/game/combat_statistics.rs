use super::CombatCounter;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CombatStatistics {
    counters: BTreeMap<CombatCounter, u64>,
    first_at_us: Option<u64>,
    last_at_us: Option<u64>,
    minimum: Option<u64>,
    maximum: Option<u64>,
    complete: bool,
}

impl Default for CombatStatistics {
    fn default() -> Self {
        Self {
            counters: BTreeMap::new(),
            first_at_us: None,
            last_at_us: None,
            minimum: None,
            maximum: None,
            complete: true,
        }
    }
}

impl CombatStatistics {
    pub(crate) fn increment(&mut self, counter: CombatCounter, value: u64) {
        let total = self.counters.entry(counter).or_default();

        if let Some(sum) = total.checked_add(value) {
            *total = sum;

            return;
        }

        self.complete = false;
    }

    pub(crate) fn touch(&mut self, at_us: u64) {
        self.first_at_us = Some(self.first_at_us.map_or(at_us, |old| old.min(at_us)));
        self.last_at_us = Some(self.last_at_us.map_or(at_us, |old| old.max(at_us)));
    }

    pub(crate) fn range(&mut self, value: u64) {
        self.minimum = Some(self.minimum.map_or(value, |old| old.min(value)));
        self.maximum = Some(self.maximum.map_or(value, |old| old.max(value)));
    }

    pub(crate) fn incomplete(&mut self) {
        self.complete = false;
    }

    pub(crate) fn complete(&self) -> bool {
        self.complete
    }

    pub(crate) fn count(&self, counter: CombatCounter) -> u64 {
        self.counters.get(&counter).copied().unwrap_or(0)
    }

    pub(crate) fn first_at_us(&self) -> Option<u64> {
        self.first_at_us
    }

    pub(crate) fn last_at_us(&self) -> Option<u64> {
        self.last_at_us
    }

    pub(crate) fn minimum(&self) -> Option<u64> {
        self.minimum
    }

    pub(crate) fn maximum(&self) -> Option<u64> {
        self.maximum
    }

    pub(crate) fn merge_owned_actions(&mut self, other: &Self) {
        let mut actions = other.clone();

        actions.counters.remove(&CombatCounter::Deaths);
        self.merge(&actions);
    }

    pub(crate) fn merge(&mut self, other: &Self) {
        self.complete &= other.complete;

        for (counter, value) in &other.counters {
            self.increment(*counter, *value);
        }

        if let Some(at) = other.first_at_us {
            self.touch(at);
        }

        if let Some(at) = other.last_at_us {
            self.touch(at);
        }

        if let Some(value) = other.minimum {
            self.range(value);
        }

        if let Some(value) = other.maximum {
            self.range(value);
        }
    }
}
