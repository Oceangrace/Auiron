use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::FormulaDefinition;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct InterfaceDocument {
    #[serde(default)]
    pub(in crate::modules::interfaces) imported: bool,
    #[serde(default)]
    pub(in crate::modules::interfaces) localization: super::InterfaceLocalization,
    #[serde(default)]
    pub(in crate::modules::interfaces) display: super::InterfaceDisplay,
    #[serde(default)]
    #[schemars(length(max = 32))]
    pub(in crate::modules::interfaces) formulas: Vec<FormulaDefinition>,
    #[schemars(length(min = 1, max = 100))]
    pub(in crate::modules::interfaces) id: String,
    #[schemars(length(min = 1, max = 128))]
    pub(in crate::modules::interfaces) name: String,
    #[serde(default)]
    #[schemars(length(max = 128))]
    pub(in crate::modules::interfaces) author: String,
    #[serde(default)]
    #[schemars(length(max = 64))]
    pub(in crate::modules::interfaces) version: String,
    #[serde(default)]
    #[schemars(length(max = 2048))]
    pub(in crate::modules::interfaces) description: String,
    pub(in crate::modules::interfaces) enabled: bool,
    #[schemars(length(max = 2000))]
    pub(in crate::modules::interfaces) roots: Vec<WidgetNode>,
}
