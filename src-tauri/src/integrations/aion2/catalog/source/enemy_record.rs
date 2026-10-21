use super::CatalogEntry;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct EnemyRecord {
    pub(in crate::integrations::aion2::catalog) entry: CatalogEntry,
    pub(in crate::integrations::aion2::catalog) context: String,
    pub(in crate::integrations::aion2::catalog) skills: Vec<CatalogEntry>,
    pub(in crate::integrations::aion2::catalog) effects: Vec<CatalogEntry>,
}
