use super::CatalogChecksum;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CatalogManifest {
    schema_version: u32,
    client_build: u32,
    catalog_version: String,
    files: HashMap<String, CatalogChecksum>,
}

impl CatalogManifest {
    pub(crate) fn parse(bytes: &[u8]) -> Result<Self, String> {
        let value: Self = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;

        if value.schema_version != 1 || value.catalog_version.is_empty() {
            return Err("Unsupported catalog manifest".into());
        }

        Ok(value)
    }

    pub(crate) fn version(&self) -> &str {
        &self.catalog_version
    }

    pub(crate) fn client_build(&self) -> u32 {
        self.client_build
    }

    pub(crate) fn verify(&self, name: &Path, bytes: &[u8]) -> Result<(), String> {
        let expected = name.to_str().and_then(|name| self.files.get(name));

        let hash = format!("{:x}", Sha256::digest(bytes));

        if expected
            .is_none_or(|expected| expected.sha256 != hash || expected.bytes != bytes.len() as u64)
        {
            return Err(format!("Catalog checksum mismatch: {}", name.display()));
        }

        Ok(())
    }
}
