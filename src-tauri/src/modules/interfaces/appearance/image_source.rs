use serde::{Deserialize, Serialize};

#[derive(
    Default, Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum ImageSource {
    #[default]
    Automatic,
    Skill,
    Effect,
    Custom,
}
