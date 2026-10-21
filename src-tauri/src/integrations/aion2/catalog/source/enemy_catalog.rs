use super::EnemyRecord;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct EnemyCatalog {
    pub(in crate::integrations::aion2::catalog) schema_version: u32,
    pub(in crate::integrations::aion2::catalog) client_build: u32,
    pub(in crate::integrations::aion2::catalog) catalog_version: String,
    pub(in crate::integrations::aion2::catalog) entries: Vec<EnemyRecord>,
}
