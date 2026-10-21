use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::num::NonZeroU32;

#[derive(Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SkillNameCatalog {
    #[schemars(range(min = 1, max = 1))]
    schema_version: u32,
    client_build: u32,
    catalog_version: String,
    skills: BTreeMap<NonZeroU32, String>,
}

impl SkillNameCatalog {
    pub(crate) fn names(&self) -> impl Iterator<Item = (u32, &str)> {
        self.skills
            .iter()
            .map(|(id, name)| (id.get(), name.as_str()))
    }

    pub(crate) fn parse(bytes: &[u8], build: u32, version: &str) -> Result<Self, String> {
        let value: Self = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;

        if value.schema_version != 1
            || value.client_build != build
            || value.catalog_version != version
            || value.skills.is_empty()
            || value.skills.len() > 100000
            || value
                .skills
                .values()
                .any(|name| name.trim().is_empty() || name.chars().count() > 256)
        {
            return Err("Invalid skill name catalog".into());
        }

        Ok(value)
    }
}
