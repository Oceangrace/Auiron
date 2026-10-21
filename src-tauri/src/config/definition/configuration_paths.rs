use std::path::{Component, Path};

pub(crate) struct ConfigurationPaths;

impl ConfigurationPaths {
    pub(crate) fn relative(path: &Path) -> Result<(), String> {
        if path.as_os_str().is_empty()
            || path
                .components()
                .any(|part| !matches!(part, Component::Normal(_)))
        {
            return Err(format!(
                "Configuration path must be relative: {}",
                path.display()
            ));
        }

        Ok(())
    }

    pub(crate) fn filename(path: &Path) -> Result<(), String> {
        Self::relative(path)?;

        if path.components().count() != 1 {
            return Err(format!(
                "Expected a configured filename: {}",
                path.display()
            ));
        }

        Ok(())
    }
}
