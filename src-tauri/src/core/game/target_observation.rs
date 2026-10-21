use super::EntityId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", content = "entityId", rename_all = "kebab-case")]
pub(crate) enum TargetObservation {
    Unknown,
    Cleared,
    Selected(EntityId),
    LastSkillRecipient(EntityId),
}

impl TargetObservation {
    pub(crate) fn entity_id(&self) -> Option<EntityId> {
        match self {
            Self::Unknown | Self::Cleared => None,
            Self::Selected(id) | Self::LastSkillRecipient(id) => Some(*id),
        }
    }
}
