use super::definition::ApplicationDefinition;
use super::definition::CatalogConfiguration;
use super::definition::LocalizationConfiguration;
#[cfg(feature = "schema")]
use super::definition::SchemaConfiguration;
use super::definition::StorageConfiguration;
use crate::core::localization::LanguagePack;
use crate::integrations::aion2::catalog::CatalogManifest;
use crate::integrations::aion2::catalog::NpcIndex;
use crate::integrations::aion2::catalog::RuntimeCatalog;
use crate::integrations::aion2::catalog::SkillNameCatalog;
use crate::integrations::aion2::catalog::ZoneCatalog;
use crate::integrations::aion2::protocol::OpcodeTable;
#[cfg(feature = "schema")]
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

pub(crate) struct ApplicationConfiguration {
    definition: ApplicationDefinition,
}

impl ApplicationConfiguration {
    pub(crate) fn load() -> Result<Self, String> {
        let definition =
            ApplicationDefinition::parse(include_bytes!(env!("AURION_CONFIGURATION")))?;

        Ok(Self { definition })
    }

    pub(crate) fn storage_directory(&self, default: PathBuf) -> Result<PathBuf, String> {
        let Some(path) = std::env::var_os("AURION_DATA_DIRECTORY") else {
            return Ok(default);
        };
        let path = PathBuf::from(path);

        if !path.is_absolute() {
            return Err("AURION_DATA_DIRECTORY must be absolute".into());
        }

        Ok(path)
    }

    pub(crate) fn icon_manifest(&self) -> &'static [u8] {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../data/game-icon-audit.json"
        ))
    }

    pub(crate) fn combat_idle_ms(&self) -> u64 {
        self.definition.protocol().combat_idle_ms()
    }

    pub(crate) fn capture(&self) -> &super::CaptureConfiguration {
        self.definition.protocol().capture()
    }

    pub(crate) fn localization(&self) -> &LocalizationConfiguration {
        self.definition.localization()
    }

    pub(crate) fn catalog(&self) -> &CatalogConfiguration {
        self.definition.catalog()
    }

    pub(crate) fn storage(&self) -> &StorageConfiguration {
        self.definition.storage()
    }

    #[cfg(feature = "schema")]
    pub(crate) fn schemas(&self) -> &SchemaConfiguration {
        self.definition.schemas()
    }

    #[cfg(feature = "schema")]
    pub(crate) fn project_root(&self) -> &Path {
        Path::new(env!("AURION_PROJECT_ROOT"))
    }

    pub(crate) fn tray_icon(&self) -> &'static [u8] {
        include_bytes!(env!("AURION_TRAY_ICON"))
    }

    pub(crate) fn packs(&self) -> Result<Vec<LanguagePack>, String> {
        serde_json::from_slice(include_bytes!(env!("AURION_BUNDLED_LOCALES")))
            .map_err(|error| error.to_string())
    }

    pub(crate) fn runtime_catalog(&self) -> Result<Arc<RuntimeCatalog>, String> {
        let manifest = CatalogManifest::parse(self.manifest())?;
        let bytes = include_bytes!(env!("AURION_CATALOG_RUNTIME"));

        manifest.verify(self.catalog().runtime(), bytes)?;

        Ok(Arc::new(RuntimeCatalog::parse(
            bytes,
            self.catalog().client_build(),
            manifest.version(),
        )?))
    }

    pub(crate) fn npc_index(&self) -> Result<NpcIndex, String> {
        let manifest = CatalogManifest::parse(self.manifest())?;

        manifest.verify(
            self.catalog().npcs(),
            include_bytes!(env!("AURION_CATALOG_NPC")),
        )?;

        NpcIndex::parse(
            include_bytes!(env!("AURION_CATALOG_NPC")),
            self.catalog().client_build(),
            manifest.version(),
        )
    }

    pub(crate) fn skill_names(&self) -> Result<SkillNameCatalog, String> {
        let manifest = CatalogManifest::parse(self.manifest())?;
        let bytes = include_bytes!(env!("AURION_CATALOG_SKILL_NAMES"));

        manifest.verify(self.catalog().skill_names(), bytes)?;

        SkillNameCatalog::parse(bytes, self.catalog().client_build(), manifest.version())
    }

    pub(crate) fn protocol(&self) -> Result<OpcodeTable, String> {
        OpcodeTable::parse(
            include_bytes!(env!("AURION_PROTOCOL_TABLE")),
            self.catalog().client_build(),
        )
    }

    pub(crate) fn manifest(&self) -> &'static [u8] {
        include_bytes!(env!("AURION_CATALOG_MANIFEST"))
    }

    pub(crate) fn player(&self) -> &'static [u8] {
        include_bytes!(env!("AURION_CATALOG_PLAYER"))
    }

    pub(crate) fn enemy(&self) -> &'static [u8] {
        include_bytes!(env!("AURION_CATALOG_ENEMY"))
    }

    pub(crate) fn zones(&self) -> Result<ZoneCatalog, String> {
        let manifest = CatalogManifest::parse(self.manifest())?;

        ZoneCatalog::parse(
            include_bytes!(env!("AURION_CATALOG_ZONES")),
            self.catalog().client_build(),
            manifest.version(),
        )
    }
}
