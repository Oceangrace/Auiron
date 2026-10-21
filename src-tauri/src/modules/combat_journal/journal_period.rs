use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub(super) enum JournalPeriod {
    Current,
    Run,
    Total,
}
