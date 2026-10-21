use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::num::NonZeroU32;

#[derive(Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct NpcCatalog {
    #[schemars(range(min = 1, max = 1))]
    schema_version: u32,
    client_build: u32,
    catalog_version: String,
    #[schemars(length(min = 1, max = 100000))]
    npc_ids: Vec<NonZeroU32>,
}

impl NpcCatalog {
    pub(crate) fn parse(bytes: &[u8], build: u32, version: &str) -> Result<Self, String> {
        let catalog: Self = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;

        if catalog.schema_version != 1
            || catalog.client_build != build
            || catalog.catalog_version != version
            || catalog.npc_ids.is_empty()
            || catalog.npc_ids.len() > 100000
            || catalog.npc_ids.iter().collect::<HashSet<_>>().len() != catalog.npc_ids.len()
        {
            return Err("Invalid NPC identifier catalog".into());
        }

        Ok(catalog)
    }

    pub(crate) fn ids(&self) -> &[NonZeroU32] {
        &self.npc_ids
    }
}
