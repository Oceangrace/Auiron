use crate::modules::interfaces::document::InterfaceWorkspace;
use crate::modules::interfaces::persistence::profile_directory::ProfileDirectory;
use crate::modules::interfaces::persistence::profile_files::ProfileFiles;
use crate::modules::interfaces::persistence::profile_index::ProfileIndex;
use crate::modules::interfaces::validation::InterfaceValidator;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

pub(crate) struct InterfaceRepository {
    profiles: PathBuf,
}

impl InterfaceRepository {
    pub(crate) fn new(profiles: PathBuf) -> Self {
        Self { profiles }
    }

    pub(crate) fn load(&self) -> Result<InterfaceWorkspace, String> {
        let _lock = self.lock()?;

        self.read()
    }

    fn lock(&self) -> Result<File, String> {
        ProfileDirectory::new(self.profiles.clone()).lock()
    }

    fn read(&self) -> Result<InterfaceWorkspace, String> {
        ProfileDirectory::new(self.profiles.clone()).recover()?;

        if !self
            .profiles
            .try_exists()
            .map_err(|error| error.to_string())?
        {
            return Ok(InterfaceWorkspace::empty());
        }

        ProfileIndex::load(&self.profiles)
    }

    pub(crate) fn save(
        &self,
        workspace: &InterfaceWorkspace,
        expected: &InterfaceWorkspace,
    ) -> Result<(), String> {
        let _lock = self.lock()?;
        let current = self.read()?;

        if serde_json::to_value(&current).map_err(|error| error.to_string())?
            != serde_json::to_value(expected).map_err(|error| error.to_string())?
        {
            return Err(
                "Save failed: the file changed externally. Your edits remain in the open editor."
                    .into(),
            );
        }

        InterfaceValidator::new().validate(workspace)?;
        ProfileFiles::limit(workspace)?;

        self.save_profiles(&self.profiles, workspace, expected)
    }

    fn save_profiles(
        &self,
        profiles: &Path,
        workspace: &InterfaceWorkspace,
        expected: &InterfaceWorkspace,
    ) -> Result<(), String> {
        let mut directory = ProfileDirectory::new(profiles.to_path_buf());
        let stage = directory.stage()?;
        let result = ProfileIndex::save(&stage, workspace).and_then(|()| {
            let current = self.read()?;

            if serde_json::to_value(&current).map_err(|error| error.to_string())?
                != serde_json::to_value(expected).map_err(|error| error.to_string())?
            {
                return Err(
                    "Profiles changed externally while saving. Your edits remain in the editor."
                        .into(),
                );
            }

            directory.commit(&stage)
        });

        if result.is_err() && stage.exists() {
            let _ = directory.discard(&stage);
        }

        result
    }
}
