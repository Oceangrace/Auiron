//! Строгое описание конфигурации, общее для сборки и приложения.

mod application_definition;
mod bundled_locale;
mod capture_configuration;
mod catalog_configuration;
mod configuration_paths;
mod language_package_configuration;
mod localization_configuration;
mod protocol_configuration;
mod resource_configuration;
mod schema_configuration;
mod storage_configuration;

pub(crate) use self::application_definition::ApplicationDefinition;

pub(crate) use self::bundled_locale::BundledLocale;

pub(crate) use self::capture_configuration::CaptureConfiguration;

pub(crate) use self::catalog_configuration::CatalogConfiguration;

pub(crate) use self::configuration_paths::ConfigurationPaths;

pub(crate) use self::language_package_configuration::LanguagePackageConfiguration;

pub(crate) use self::localization_configuration::LocalizationConfiguration;

pub(crate) use self::protocol_configuration::ProtocolConfiguration;

pub(crate) use self::resource_configuration::ResourceConfiguration;

pub(crate) use self::schema_configuration::SchemaConfiguration;

pub(crate) use self::storage_configuration::StorageConfiguration;
