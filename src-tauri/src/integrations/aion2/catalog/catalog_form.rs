use serde::Serialize;

#[derive(Clone, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CatalogForm {
    pub(crate) key: String,
    pub(crate) name: String,
    pub(crate) match_ids: Vec<u32>,
}
