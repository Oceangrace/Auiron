use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct WidgetAppearance {
    pub(in crate::modules::interfaces) text: String,
    pub(in crate::modules::interfaces) color: String,
    pub(in crate::modules::interfaces) background: String,
    pub(in crate::modules::interfaces) border_color: String,
    pub(in crate::modules::interfaces) font_size: f64,
    pub(in crate::modules::interfaces) opacity: f64,
    pub(in crate::modules::interfaces) icon_path: Option<String>,
    #[serde(default)]
    pub(in crate::modules::interfaces) custom_image: Option<String>,
}
