use crate::integrations::aion2::catalog::CatalogItem;
use serde::Serialize;

#[derive(Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CatalogPage {
    pub(crate) catalog_version: String,
    pub(crate) total: usize,
    pub(crate) available_categories: Vec<String>,
    pub(crate) items: Vec<CatalogItem>,
}
