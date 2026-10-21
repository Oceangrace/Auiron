use crate::core::preferences::OverlayPreferences;
use std::fs::{self, File, OpenOptions};
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) struct PreferencesRepository {
    path: PathBuf,
    default_locale: String,
}

impl PreferencesRepository {
    // Bound memory use before parsing a damaged preferences file.
    const MAX_BYTES: u64 = 32 * 1024;

    pub(crate) fn new(path: PathBuf, default_locale: String) -> Self {
        Self {
            path,
            default_locale,
        }
    }

    pub(crate) fn read(&self) -> Result<OverlayPreferences, String> {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                return Ok(OverlayPreferences {
                    locale: self.default_locale.clone(),
                    ..Default::default()
                });
            }
            Err(error) => return Err(error.to_string()),
        };

        let mut bytes = Vec::new();

        file.take(Self::MAX_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| error.to_string())?;

        if bytes.len() as u64 > Self::MAX_BYTES {
            return Err("Preferences file is too large and was left unchanged".into());
        }

        let mut preferences: OverlayPreferences =
            serde_json::from_slice(&bytes).map_err(|error| {
                format!("Preferences file is damaged and was left unchanged: {error}")
            })?;

        if preferences.locale.is_empty() {
            preferences.locale.clone_from(&self.default_locale);
        }

        preferences.validate()?;

        Ok(preferences)
    }

    pub(crate) fn save(&self, preferences: OverlayPreferences) -> Result<(), String> {
        self.read()?;

        preferences.validate()?;

        fs::create_dir_all(
            self.path
                .parent()
                .ok_or("Preferences directory is missing")?,
        )
        .map_err(|error| error.to_string())?;

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| error.to_string())?
            .as_nanos();

        let temporary = self.path.with_extension(format!("{nonce}.tmp"));

        let bytes = serde_json::to_vec_pretty(&preferences).map_err(|error| error.to_string())?;

        let result = (|| {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)
                .map_err(|error| error.to_string())?;

            file.write_all(&bytes).map_err(|error| error.to_string())?;
            file.sync_all().map_err(|error| error.to_string())?;
            drop(file);

            fs::rename(&temporary, self.path.clone()).map_err(|error| error.to_string())
        })();

        if result.is_err() {
            let _ = fs::remove_file(temporary);
        }

        result
    }
}
