use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LanguageManifest {
    schema_version: u32,
    id: String,
    label: String,
}

impl LanguageManifest {
    pub(crate) fn supported(&self) -> bool {
        self.schema_version == 1
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }
}
