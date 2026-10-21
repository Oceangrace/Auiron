use crate::modules::interfaces::rules::EffectFilter;
use crate::modules::interfaces::rules::TriggerUnit;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct GroupEffectSettings {
    pub(in crate::modules::interfaces) unit: TriggerUnit,
    pub(in crate::modules::interfaces) filter: EffectFilter,
    #[schemars(range(min = 8, max = 256))]
    pub(in crate::modules::interfaces) icon_size: f64,
}
