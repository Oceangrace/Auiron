use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CatalogConfiguration {
    directory: PathBuf,
    #[schemars(range(min = 1, max = u32::MAX))]
    client_build: u32,
    manifest: PathBuf,
    player: PathBuf,
    enemy: PathBuf,
    zones: PathBuf,
    npcs: PathBuf,
    runtime: PathBuf,
    skill_names: PathBuf,
    source_locale: String,
}

impl CatalogConfiguration {
    pub(crate) fn directory(&self) -> &Path {
        &self.directory
    }

    pub(crate) fn client_build(&self) -> u32 {
        self.client_build
    }

    pub(crate) fn manifest(&self) -> &Path {
        &self.manifest
    }

    pub(crate) fn player(&self) -> &Path {
        &self.player
    }

    pub(crate) fn enemy(&self) -> &Path {
        &self.enemy
    }

    pub(crate) fn skill_names(&self) -> &Path {
        &self.skill_names
    }

    pub(crate) fn runtime(&self) -> &Path {
        &self.runtime
    }

    pub(crate) fn npcs(&self) -> &Path {
        &self.npcs
    }

    pub(crate) fn zones(&self) -> &Path {
        &self.zones
    }

    pub(crate) fn source_locale(&self) -> &str {
        &self.source_locale
    }
}
