use super::IconDownload;
use super::IconName;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct IconManifest {
    client_build: u32,
    aliases: BTreeMap<String, String>,
    unavailable: BTreeSet<String>,
}

impl IconManifest {
    pub(crate) fn plan(
        bytes: &[u8],
        build: u32,
        paths: &BTreeSet<String>,
    ) -> Result<Vec<IconDownload>, String> {
        let manifest: Self = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;

        if manifest.client_build != build
            || manifest
                .unavailable
                .iter()
                .chain(manifest.aliases.keys())
                .any(|path| !paths.contains(path))
        {
            return Err("Icon manifest does not match the catalog".into());
        }

        let mut downloads: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut owners = BTreeMap::new();

        for path in paths {
            let file = IconName::file(path)?;

            if manifest.unavailable.contains(path) {
                continue;
            }

            let mut source = path;
            let mut visited = BTreeSet::new();

            while let Some(alias) = manifest.aliases.get(source) {
                if !visited.insert(source) {
                    return Err(format!("Icon alias cycle: {path}"));
                }

                source = alias;
            }

            IconName::file(source)?;
            let name = source.rsplit('/').next().ok_or("Empty icon name")?;

            if let Some(previous) = owners.insert(file.clone(), name.to_owned())
                && previous != name
            {
                return Err(format!("Conflicting icon filenames: {path}"));
            }

            downloads.entry(name.to_owned()).or_default().insert(file);
        }

        Ok(downloads
            .into_iter()
            .map(|(name, files)| IconDownload {
                name,
                files: files.into_iter().collect(),
            })
            .collect())
    }
}
