use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CatalogChecksum {
    pub(super) sha256: String,
    pub(super) bytes: u64,
}
