use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SchemaConfiguration {
    directory: PathBuf,
    application: PathBuf,
    journal: PathBuf,
    workspace: PathBuf,
    document: PathBuf,
    triggers: PathBuf,
    formula: PathBuf,
    opcodes: PathBuf,
    npc_catalog: PathBuf,
    runtime_catalog: PathBuf,
    skill_names: PathBuf,
}

impl SchemaConfiguration {
    pub(crate) fn directory(&self) -> &Path {
        &self.directory
    }

    pub(crate) fn journal(&self) -> &Path {
        &self.journal
    }

    pub(crate) fn application(&self) -> &Path {
        &self.application
    }

    pub(crate) fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub(crate) fn document(&self) -> &Path {
        &self.document
    }

    pub(crate) fn triggers(&self) -> &Path {
        &self.triggers
    }

    pub(crate) fn skill_names(&self) -> &Path {
        &self.skill_names
    }

    pub(crate) fn runtime_catalog(&self) -> &Path {
        &self.runtime_catalog
    }

    pub(crate) fn npc_catalog(&self) -> &Path {
        &self.npc_catalog
    }

    pub(crate) fn opcodes(&self) -> &Path {
        &self.opcodes
    }

    pub(crate) fn formula(&self) -> &Path {
        &self.formula
    }
}
