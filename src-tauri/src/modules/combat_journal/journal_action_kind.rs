use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum JournalActionKind {
    Damage,
    Healing,
    Cast,
    Trigger,
    CastStart,
    Miss,
    Resist,
    Death,
}
