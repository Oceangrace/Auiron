use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum WorldBossStatus {
    Waiting,
    Countdown,
    ObservedAlive,
    Refresh,
}
