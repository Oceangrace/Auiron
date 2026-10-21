use super::server_clock::ServerClock;
use crate::core::game::EffectSnapshot;
use crate::core::game::EffectTiming;
use crate::core::game::ObservedEffect;

pub(super) struct EffectState {
    effect: ObservedEffect,
    first_seen_us: u64,
}

impl EffectState {
    pub(super) fn new(effect: ObservedEffect, first_seen_us: u64) -> Self {
        Self {
            effect,
            first_seen_us,
        }
    }

    pub(super) fn effect(&self) -> &ObservedEffect {
        &self.effect
    }

    pub(super) fn first_seen_us(&self) -> u64 {
        self.first_seen_us
    }

    pub(super) fn snapshot(&self, clock: &ServerClock, now_us: u64) -> Option<EffectSnapshot> {
        let (duration, expires) = match self.effect.timing() {
            EffectTiming::Untimed => (None, None),
            EffectTiming::Timed {
                duration_ms,
                ends_at_server_ms,
            } => (
                Some(*duration_ms),
                ends_at_server_ms.and_then(|end| clock.to_local(end)),
            ),
        };

        if expires.is_some_and(|end| end <= now_us) {
            return None;
        }

        let applied = expires
            .zip(duration)
            .and_then(|(end, duration)| end.checked_sub(duration.saturating_mul(1000)))
            .unwrap_or(self.first_seen_us);

        Some(EffectSnapshot::new(self.effect.clone(), applied, expires))
    }
}
