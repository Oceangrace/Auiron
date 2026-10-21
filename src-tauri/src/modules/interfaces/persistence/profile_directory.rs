use crate::modules::interfaces::persistence::profile_files::ProfileFiles;
use std::fs;
use std::fs::OpenOptions;
use std::os::windows::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

pub(crate) struct ProfileDirectory {
    path: PathBuf,
    backup: PathBuf,
    staged: Option<PathBuf>,
}

impl ProfileDirectory {
    pub(crate) fn new(path: PathBuf) -> Self {
        let backup = Self::sibling(&path, ".backup");

        Self {
            path,
            backup,
            staged: None,
        }
    }

    fn sibling(path: &Path, suffix: &str) -> PathBuf {
        let mut filename = path.file_name().unwrap_or_default().to_os_string();

        filename.push(suffix);

        path.with_file_name(filename)
    }

    pub(crate) fn lock(&self) -> Result<fs::File, String> {
        let parent = self
            .path
            .parent()
            .ok_or("Profile parent directory is missing")?;

        fs::create_dir_all(parent).map_err(|error| error.to_string())?;

        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .share_mode(0)
            .open(Self::sibling(&self.path, ".lock"))
            .map_err(|error| {
                format!("Could not lock profiles; another operation may be using them: {error}")
            })
    }

    pub(crate) fn recover(&self) -> Result<(), String> {
        if !self.path.try_exists().map_err(|error| error.to_string())?
            && self
                .backup
                .try_exists()
                .map_err(|error| error.to_string())?
        {
            ProfileFiles::regular(&self.backup, true)?;
            fs::rename(&self.backup, &self.path).map_err(|error| error.to_string())?;
        }

        Ok(())
    }

    pub(crate) fn stage(&mut self) -> Result<PathBuf, String> {
        if self.staged.is_some() {
            return Err("A profile staging directory already exists".into());
        }

        let parent = self
            .path
            .parent()
            .ok_or("Profile parent directory is missing")?;

        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        let stage = parent.join(format!(".profiles-{}.tmp", uuid::Uuid::new_v4()));

        fs::create_dir(&stage).map_err(|error| error.to_string())?;
        // Windows may redirect an individual AppData child without redirecting its parent.
        // Retain the actual directory we created as the boundary for this transaction.
        self.staged = Some(fs::canonicalize(&stage).map_err(|error| error.to_string())?);

        Ok(stage)
    }

    pub(crate) fn commit(&self, stage: &Path) -> Result<(), String> {
        let resolved = fs::canonicalize(stage).map_err(|error| error.to_string())?;

        if self.staged.as_ref() != Some(&resolved) {
            return Err("Profile staging directory does not belong to this transaction".into());
        }

        if self
            .backup
            .try_exists()
            .map_err(|error| error.to_string())?
        {
            self.discard(&self.backup)?;
        }

        let exists = self.path.try_exists().map_err(|error| error.to_string())?;

        if exists {
            ProfileFiles::regular(&self.path, true)?;
            fs::rename(&self.path, &self.backup).map_err(|error| error.to_string())?;
        }

        if let Err(error) = fs::rename(stage, &self.path) {
            self.recover()?;

            return Err(format!("Could not commit profiles: {error}"));
        }

        Ok(())
    }

    pub(crate) fn discard(&self, path: &Path) -> Result<(), String> {
        ProfileFiles::regular(path, true)?;
        let parent = self
            .path
            .parent()
            .ok_or("Profile parent directory is missing")?;
        let resolved_parent = fs::canonicalize(parent).map_err(|error| error.to_string())?;
        let resolved = fs::canonicalize(path).map_err(|error| error.to_string())?;
        let staged = self
            .staged
            .as_ref()
            .ok_or("Profile staging directory is missing")?;

        let owned = (path == self.backup || &resolved == staged) && path.parent() == Some(parent);
        let inside = resolved.parent() == Some(resolved_parent.as_path())
            || resolved.parent() == staged.parent();

        if !owned || !inside || resolved == resolved_parent {
            return Err("Temporary profile path is outside its storage directory".into());
        }

        fs::remove_dir_all(&resolved).map_err(|error| error.to_string())
    }
}
