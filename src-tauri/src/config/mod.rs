//! Конфигурация приложения и ресурсы, выбранные при сборке.

mod application_configuration;
mod definition;

pub(crate) use self::application_configuration::ApplicationConfiguration;
#[cfg(feature = "schema")]
pub(crate) use self::definition::ApplicationDefinition;
pub(crate) use self::definition::CaptureConfiguration;
pub(crate) use self::definition::CatalogConfiguration;
pub(crate) use self::definition::LanguagePackageConfiguration;
