//! Строгие контракты файлов справочника приложения.

mod catalog_checksum;
mod catalog_choice;
mod catalog_entry;
mod catalog_manifest;
mod enemy_catalog;
mod enemy_record;
mod npc_catalog;
mod player_catalog;
mod zone_catalog;

pub(super) use self::catalog_checksum::CatalogChecksum;
pub(super) use self::catalog_choice::CatalogChoice;
pub(super) use self::catalog_entry::CatalogEntry;
pub(crate) use self::catalog_manifest::CatalogManifest;
pub(super) use self::enemy_catalog::EnemyCatalog;
pub(super) use self::enemy_record::EnemyRecord;
pub(crate) use self::npc_catalog::NpcCatalog;
pub(super) use self::player_catalog::PlayerCatalog;
pub(crate) use self::zone_catalog::ZoneCatalog;

mod skill_name_catalog;
pub(crate) use self::skill_name_catalog::SkillNameCatalog;
