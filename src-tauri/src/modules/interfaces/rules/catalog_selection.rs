use crate::integrations::aion2::catalog::CatalogScope;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CatalogSelection {
    pub(in crate::modules::interfaces) catalog_version: String,
    pub(in crate::modules::interfaces) scope: CatalogScope,
    pub(in crate::modules::interfaces) entry_key: String,
    pub(in crate::modules::interfaces) choice_key: String,
    pub(in crate::modules::interfaces) context_key: Option<String>,
}
