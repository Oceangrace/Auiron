use crate::modules::interfaces::document::DocumentFormat;
use crate::modules::interfaces::document::InterfaceWorkspace;
use crate::modules::interfaces::persistence::profile_files::ProfileFiles;
use crate::modules::interfaces::validation::InterfaceValidator;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ProfileIndex {
    format_version: u32,
    revision: u64,
    profiles: Vec<String>,
}

impl ProfileIndex {
    pub(super) fn load(directory: &Path) -> Result<InterfaceWorkspace, String> {
        ProfileFiles::regular(directory, true)?;
        let index: Self = ProfileFiles::read(&directory.join("workspace.json"))?;

        if index.format_version != DocumentFormat::VERSION || index.profiles.len() > 100 {
            return Err("Unsupported profile index or too many profiles".into());
        }

        let mut workspace = InterfaceWorkspace::empty();
        let mut names = HashSet::new();

        workspace.revision = index.revision;

        for folder in index.profiles {
            ProfileFiles::component(&folder)?;

            if !names.insert(folder.to_lowercase()) {
                return Err("Duplicate profile folder in workspace index".into());
            }

            workspace
                .documents
                .push(ProfileFiles::load(&directory.join(folder))?);
            ProfileFiles::limit(&workspace)?;
        }

        InterfaceValidator::new().validate(&workspace)?;

        Ok(workspace)
    }

    pub(super) fn save(directory: &Path, workspace: &InterfaceWorkspace) -> Result<(), String> {
        let mut profiles = Vec::new();
        let mut names = HashSet::from(["workspace.json".to_owned()]);

        for document in &workspace.documents {
            let base = ProfileFiles::folder(&document.name);
            let mut folder = base.clone();
            let mut number = 1;

            while !names.insert(folder.to_lowercase()) {
                number += 1;
                folder = format!("{} ({number})", base.chars().take(56).collect::<String>());
            }

            ProfileFiles::save(&directory.join(&folder), document)?;
            profiles.push(folder);
        }

        ProfileFiles::write(
            &directory.join("workspace.json"),
            &Self {
                format_version: workspace.format_version,
                revision: workspace.revision,
                profiles,
            },
        )
    }
}
