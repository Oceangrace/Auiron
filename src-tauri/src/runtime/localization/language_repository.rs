use crate::config::LanguagePackageConfiguration;
use crate::core::localization::LanguageManifest;
use crate::core::localization::LanguagePack;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::path::Path;

pub(crate) struct LanguageRepository;

impl LanguageRepository {
    pub(crate) fn load(
        directory: &Path,
        bundled: &mut Vec<LanguagePack>,
        files: &LanguagePackageConfiguration,
        version: &str,
    ) -> Vec<String> {
        let mut warnings = Vec::new();

        let entries = match std::fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(error) if error.kind() == ErrorKind::NotFound => return warnings,
            Err(error) => return vec![error.to_string()],
        };

        for entry in entries.take(64) {
            let result = entry.map_err(|error| error.to_string()).and_then(|entry| {
                if !entry
                    .file_type()
                    .map_err(|error| error.to_string())?
                    .is_dir()
                {
                    return Ok(());
                }

                let pack = Self::read(&entry.path(), files, version)
                    .map_err(|error| format!("{}: {error}", entry.path().display()))?;

                if bundled.iter().any(|item| item.id() == pack.id()) {
                    return Err(format!("Language {} is already installed", pack.id()));
                }

                bundled.push(pack);

                Ok(())
            });

            if let Err(error) = result {
                warnings.push(error);
            }
        }

        warnings
    }

    fn read(
        directory: &Path,
        files: &LanguagePackageConfiguration,
        version: &str,
    ) -> Result<LanguagePack, String> {
        let manifest: LanguageManifest = Self::json(&directory.join(files.manifest()))?;

        if !manifest.supported() {
            return Err(format!(
                "Unsupported language pack: {}",
                directory.display()
            ));
        }

        let messages = Self::json(&directory.join(files.messages()))?;

        let mut catalog = Value::Null;

        if directory.join(files.catalog()).exists() {
            catalog = Self::json(&directory.join(files.catalog()))?;

            if catalog["catalogVersion"] != version {
                return Err(format!(
                    "Language catalog version mismatch: {}",
                    directory.display()
                ));
            }
        }

        let zones = catalog["zones"].clone();

        let pack = LanguagePack::new(
            manifest.id().to_owned(),
            manifest.label().to_owned(),
            messages,
            catalog,
            zones,
        );

        pack.validate()?;

        Ok(pack)
    }

    fn json<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
        let file = File::open(path).map_err(|error| format!("{}: {error}", path.display()))?;

        let mut bytes = Vec::new();

        file.take(8_000_001)
            .read_to_end(&mut bytes)
            .map_err(|error| error.to_string())?;

        if bytes.len() > 8_000_000 {
            return Err("Language file exceeds 8 MB".into());
        }

        serde_json::from_slice(&bytes).map_err(|error| format!("{}: {error}", path.display()))
    }
}
