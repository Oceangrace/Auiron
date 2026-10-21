use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum ClassColorSource {
    #[default]
    None,
    Player,
    Target,
    TargetTarget,
    Entity,
    Row,
}

impl ClassColorSource {
    pub(crate) fn field(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Player => Some("player.class"),
            Self::Target => Some("target.class"),
            Self::TargetTarget => Some("targetTarget.class"),
            Self::Entity => Some("entity.class"),
            Self::Row => Some("row.class"),
        }
    }
}
