use crate::integrations::aion2::catalog::CatalogItem;
use serde::Serialize;

#[derive(Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CatalogRecord {
    pub(crate) catalog_version: String,
    pub(crate) item: Option<CatalogItem>,
}
