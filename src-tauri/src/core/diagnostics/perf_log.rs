use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Aggregates the cost of one hot path and appends a JSON line every ten seconds to the file
/// named by `AURION_PERF_LOG`. Without the variable every call returns at once.
pub(crate) struct PerfLog {
    name: &'static str,
    path: Option<PathBuf>,
    since: Instant,
    calls: u64,
    sent: u64,
    total_us: u128,
    max_us: u128,
    bytes: u64,
}

impl PerfLog {
    const WINDOW: Duration = Duration::from_secs(10);

    pub(crate) fn new(name: &'static str) -> Self {
        Self {
            name,
            path: std::env::var_os("AURION_PERF_LOG").map(PathBuf::from),
            since: Instant::now(),
            calls: 0,
            sent: 0,
            total_us: 0,
            max_us: 0,
            bytes: 0,
        }
    }

    pub(crate) fn enabled(&self) -> bool {
        self.path.is_some()
    }

    pub(crate) fn measure<T>(
        name: &'static str,
        operation: impl FnOnce() -> Result<T, String>,
    ) -> Result<T, String> {
        let path = std::env::var_os("AURION_PERF_LOG");
        let started = Instant::now();
        let result = operation();
        let elapsed = started.elapsed();

        if let Some(path) = path {
            let at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |time| time.as_millis());
            let line = serde_json::json!({
                "atMs": at,
                "name": name,
                "elapsedUs": elapsed.as_micros() as u64,
                "ok": result.is_ok(),
            });

            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                let _ = writeln!(file, "{line}");
            }
        }

        result
    }

    pub(crate) fn record(&mut self, elapsed: Duration, bytes: Option<usize>) {
        if !self.enabled() {
            return;
        }

        let micros = elapsed.as_micros();

        self.calls += 1;
        self.total_us += micros;
        self.max_us = self.max_us.max(micros);

        if let Some(bytes) = bytes {
            self.sent += 1;
            self.bytes += bytes as u64;
        }

        if self.since.elapsed() >= Self::WINDOW {
            self.flush();
        }
    }

    fn flush(&mut self) {
        let at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |time| time.as_millis());

        let line = serde_json::json!({
            "atMs": at,
            "name": self.name,
            "seconds": self.since.elapsed().as_secs_f64(),
            "calls": self.calls,
            "sent": self.sent,
            "avgUs": self.total_us.checked_div(u128::from(self.calls)).unwrap_or(0) as u64,
            "maxUs": self.max_us as u64,
            "sentKb": self.bytes / 1024,
        });

        // Diagnostics must never disturb the overlay: a failed write is dropped.
        if let Some(path) = &self.path
            && let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path)
        {
            let _ = writeln!(file, "{line}");
        }

        *self = Self {
            path: self.path.take(),
            ..Self::new(self.name)
        };
    }
}
