use super::CatalogConfiguration;
use super::LocalizationConfiguration;
use super::ProtocolConfiguration;
use super::ResourceConfiguration;
use super::SchemaConfiguration;
use super::StorageConfiguration;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ApplicationDefinition {
    catalog: CatalogConfiguration,
    protocol: ProtocolConfiguration,
    localization: LocalizationConfiguration,
    resources: ResourceConfiguration,
    schemas: SchemaConfiguration,
    storage: StorageConfiguration,
}

impl ApplicationDefinition {
    pub(crate) fn parse(bytes: &[u8]) -> Result<Self, String> {
        let value: Self = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;

        value.validate()?;

        Ok(value)
    }

    fn validate(&self) -> Result<(), String> {
        self.protocol().validate()?;

        for path in [
            self.catalog.directory(),
            self.protocol().table(),
            self.localization.directory(),
            self.resources().tray_icon(),
            self.schemas().directory(),
        ] {
            super::ConfigurationPaths::relative(path)?;
        }

        for path in [
            self.catalog.manifest(),
            self.catalog.player(),
            self.catalog.enemy(),
            self.catalog.zones(),
            self.catalog.npcs(),
            self.catalog.runtime(),
            self.catalog.skill_names(),
            self.storage.preferences(),
            self.storage.profiles(),
            self.storage.journal(),
            self.localization.package().manifest(),
            self.localization.package().messages(),
            self.localization.package().catalog(),
            self.schemas.application(),
            self.schemas.journal(),
            self.schemas.workspace(),
            self.schemas.document(),
            self.schemas.triggers(),
            self.schemas.formula(),
            self.schemas.opcodes(),
            self.schemas.npc_catalog(),
            self.schemas.runtime_catalog(),
            self.schemas.skill_names(),
        ] {
            super::ConfigurationPaths::filename(path)?;
        }

        for locale in self.localization.bundled() {
            if locale.id().is_empty()
                || locale.id().len() > 63
                || locale.label().trim().is_empty()
                || locale.label().chars().count() > 80
            {
                return Err("Invalid bundled language configuration".into());
            }

            super::ConfigurationPaths::relative(locale.ui())?;

            if let Some(path) = locale.catalog() {
                super::ConfigurationPaths::relative(path)?;
            }
        }

        if [
            self.storage.preferences(),
            self.storage.profiles(),
            self.storage.journal(),
        ]
        .into_iter()
        .collect::<HashSet<_>>()
        .len()
            != 3
        {
            return Err("Storage filenames must be distinct".into());
        }

        let languages = self
            .localization
            .bundled()
            .iter()
            .map(|locale| locale.id())
            .collect::<HashSet<_>>();

        if languages.len() != self.localization.bundled().len()
            || !languages.contains(self.localization.default_locale())
            || !languages.contains(self.localization.fallback_locale())
            || self.catalog.client_build() == 0
            || self.catalog.source_locale().is_empty()
        {
            return Err("Invalid configured catalog build or languages".into());
        }

        let schema_files = [
            self.schemas.application(),
            self.schemas.journal(),
            self.schemas.workspace(),
            self.schemas.document(),
            self.schemas.triggers(),
            self.schemas.formula(),
            self.schemas.opcodes(),
            self.schemas.npc_catalog(),
            self.schemas.runtime_catalog(),
            self.schemas.skill_names(),
        ];

        if schema_files.iter().collect::<HashSet<_>>().len() != schema_files.len() {
            return Err("Schema filenames must be distinct".into());
        }

        Ok(())
    }

    pub(crate) fn protocol(&self) -> &ProtocolConfiguration {
        &self.protocol
    }

    pub(crate) fn catalog(&self) -> &CatalogConfiguration {
        &self.catalog
    }

    pub(crate) fn localization(&self) -> &LocalizationConfiguration {
        &self.localization
    }

    pub(crate) fn resources(&self) -> &ResourceConfiguration {
        &self.resources
    }

    pub(crate) fn schemas(&self) -> &SchemaConfiguration {
        &self.schemas
    }

    pub(crate) fn storage(&self) -> &StorageConfiguration {
        &self.storage
    }
}
