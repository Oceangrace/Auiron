use serde::Deserialize;
use std::collections::BTreeMap;
use std::num::NonZeroU32;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ZoneCatalog {
    schema_version: u32,
    client_build: u32,
    catalog_version: String,
    zones: BTreeMap<String, Vec<NonZeroU32>>,
}

impl ZoneCatalog {
    pub(crate) fn parse(bytes: &[u8], build: u32, version: &str) -> Result<Self, String> {
        let catalog: Self = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;

        if catalog.schema_version != 1
            || catalog.client_build != build
            || catalog.catalog_version != version
            || catalog
                .zones
                .iter()
                .any(|(name, ids)| name.trim().is_empty() || ids.is_empty())
        {
            return Err("Invalid zone catalog".into());
        }

        Ok(catalog)
    }

    pub(crate) fn name(&self, map_id: u32) -> Option<&str> {
        self.zones
            .iter()
            .find(|(_, ids)| ids.iter().any(|id| id.get() == map_id))
            .map(|(name, _)| name.as_str())
    }

    pub(crate) fn values(&self) -> Vec<String> {
        self.zones.keys().cloned().collect()
    }

    pub(crate) fn maps(&self) -> impl Iterator<Item = (u32, &str)> {
        self.zones
            .iter()
            .flat_map(|(name, ids)| ids.iter().map(move |id| (id.get(), name.as_str())))
    }
}
