use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum HitOutcome {
    Normal,
    Critical,
    Miss,
    Resist,
}
