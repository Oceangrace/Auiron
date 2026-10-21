use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WidgetKind {
    Icon,
    Text,
    Group,
    Bar,
    DynamicGroup,
}
