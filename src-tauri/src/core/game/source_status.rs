use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum SourceStatus {
    Offline,
    Live,
    Replay,
    Stale,
}
