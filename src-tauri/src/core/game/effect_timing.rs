use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase"
)]
pub(crate) enum EffectTiming {
    Untimed,
    Timed {
        duration_ms: u64,
        ends_at_server_ms: Option<u64>,
    },
}
