use crate::modules::interfaces::rules::CatalogSelection;
use crate::modules::interfaces::rules::ObservationKind;
use crate::modules::interfaces::rules::TriggerSubject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct TriggerDefinition {
    pub(in crate::modules::interfaces) subject: TriggerSubject,
    pub(in crate::modules::interfaces) observation: ObservationKind,
    pub(in crate::modules::interfaces) actor_key: Option<String>,
    pub(in crate::modules::interfaces) selection: CatalogSelection,
    pub(in crate::modules::interfaces) display_seconds: Option<f64>,
}
