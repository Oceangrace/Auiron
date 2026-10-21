use super::GroupButtonDisplay;
use super::GroupButtonPlacement;
use super::GroupHeaderAction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct GroupHeaderButton {
    #[schemars(length(min = 1, max = 100))]
    pub(in crate::modules::interfaces) id: String,
    #[schemars(length(max = 128))]
    pub(in crate::modules::interfaces) label: String,
    pub(in crate::modules::interfaces) display: GroupButtonDisplay,
    pub(in crate::modules::interfaces) placement: GroupButtonPlacement,
    pub(in crate::modules::interfaces) action: GroupHeaderAction,
    #[schemars(length(min = 1, max = 200))]
    pub(in crate::modules::interfaces) icon: String,
}
