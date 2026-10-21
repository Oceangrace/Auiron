use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct WidgetLayout {
    #[schemars(range(min = -100_000, max = 100_000))]
    pub(in crate::modules::interfaces) x: f64,
    #[schemars(range(min = -100_000, max = 100_000))]
    pub(in crate::modules::interfaces) y: f64,
    #[schemars(range(min = 8, max = 8192))]
    pub(in crate::modules::interfaces) width: f64,
    #[schemars(range(min = 8, max = 8192))]
    pub(in crate::modules::interfaces) height: f64,
}
