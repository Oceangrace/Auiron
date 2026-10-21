use crate::modules::interfaces::document::InterfaceDocument;
use crate::modules::interfaces::persistence::ProfileDirectory;
use crate::modules::interfaces::persistence::ProfileFiles;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) struct ProfilePackage;

impl ProfilePackage {
    pub(crate) fn export(parent: &Path, document: &InterfaceDocument) -> Result<PathBuf, String> {
        let name = ProfileFiles::folder(&document.name);
        let target = parent.join(&name);

        if target.try_exists().map_err(|error| error.to_string())? {
            return Err(format!(
                "Profile folder already exists; choose another destination: {}",
                target.display()
            ));
        }

        let mut directory = ProfileDirectory::new(target.clone());
        let stage = directory.stage()?;
        let payload = stage.join(&name);
        let result = ProfileFiles::save(&payload, document)
            .and_then(|()| fs::rename(&payload, &target).map_err(|error| error.to_string()));

        let _ = directory.discard(&stage);
        result?;

        Ok(target)
    }

    pub(crate) fn import(path: &Path) -> Result<InterfaceDocument, String> {
        let document = ProfileFiles::load(path)?;

        ProfileFiles::validate(&document)?;

        Ok(document)
    }
}
