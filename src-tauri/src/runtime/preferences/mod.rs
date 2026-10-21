//! Хранение настроек оболочки и команды доступа к ним.

pub(crate) mod preferences_commands;
mod preferences_repository;

pub(crate) use self::preferences_repository::PreferencesRepository;
