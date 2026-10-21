use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum TableDimension {
    #[serde(rename = "combatants")]
    Combatants,
    #[serde(rename = "skills")]
    Skills,
    #[serde(rename = "targets")]
    Targets,
}
