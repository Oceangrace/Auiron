use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Marks that the page is alive: it polls the HUD every 100–500 ms. Quitting is normally
/// done by the page so unsaved edits are saved first; a silent page cannot do that.
pub(crate) struct PageHeartbeat {
    started: Instant,
    last_ms: AtomicU64,
}

impl PageHeartbeat {
    const SILENCE: Duration = Duration::from_secs(5);

    pub(crate) fn new() -> Self {
        Self {
            started: Instant::now(),
            last_ms: AtomicU64::new(0),
        }
    }

    pub(crate) fn beat(&self) {
        let now = self.started.elapsed().as_millis() as u64;

        self.last_ms.store(now.max(1), Ordering::Relaxed);
    }

    pub(crate) fn responsive(&self) -> bool {
        let last = self.last_ms.load(Ordering::Relaxed);

        last != 0
            && (self.started.elapsed().as_millis() as u64).saturating_sub(last)
                < Self::SILENCE.as_millis() as u64
    }
}
