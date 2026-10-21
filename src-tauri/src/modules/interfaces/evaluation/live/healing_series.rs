use std::collections::VecDeque;

/// A short, per-application sample. No cross-character or cross-effect calibration.
#[derive(Default)]
pub(super) struct HealingSeries {
    ticks: VecDeque<(u64, u32)>,
    pool: Option<u32>,
}

impl HealingSeries {
    pub(super) fn announce(&mut self, remaining: u32) {
        if self.ticks.is_empty() {
            self.pool = Some(remaining);
        }
    }

    /// `pool` is the server-reported remaining amount after this tick, when the message has it.
    pub(super) fn observe(&mut self, at_us: u64, amount: u32, pool: Option<u32>) {
        if amount == 0
            || self
                .ticks
                .back()
                .is_some_and(|(previous, _)| at_us <= *previous)
        {
            return;
        }

        self.ticks.push_back((at_us, amount));
        self.pool = pool;

        if self.ticks.len() > 3 {
            self.ticks.pop_front();
        }
    }

    pub(super) fn remaining(&self, now_us: u64, expires_us: u64) -> Option<f64> {
        if now_us >= expires_us {
            return None;
        }

        if let Some(pool) = self.pool {
            if self.ticks.len() >= 2 {
                let last_at = self.ticks.back()?.0;

                let previous_at = self.ticks[self.ticks.len() - 2].0;

                let interval = last_at.checked_sub(previous_at)?;

                if now_us.saturating_sub(last_at) > interval.saturating_mul(5) / 4 {
                    return None;
                }
            }

            return Some(f64::from(pool));
        }

        if self.ticks.len() != 3 {
            return None;
        }

        let first_at = self.ticks[0].0;

        let middle_at = self.ticks[1].0;

        let last_at = self.ticks[2].0;

        let interval = last_at.checked_sub(middle_at)?;

        let previous_interval = middle_at.checked_sub(first_at)?;

        if interval == 0 || interval.abs_diff(previous_interval) > interval / 4 {
            return None;
        }

        // A missing expected tick makes the estimate unknown until observations resume.
        if now_us.saturating_sub(last_at) > interval.saturating_mul(5) / 4 {
            return None;
        }

        let count = expires_us.saturating_sub(last_at) / interval;

        if count > 1024 {
            return None;
        }

        let amount = self
            .ticks
            .iter()
            .map(|(_, gain)| f64::from(*gain))
            .sum::<f64>()
            / self.ticks.len() as f64;
        let ticks = (1..=count)
            .filter(|index| last_at.saturating_add(index * interval) > now_us)
            .count();

        Some(amount * ticks as f64)
    }
}
