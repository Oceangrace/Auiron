//! Чтение пользовательских языковых пакетов и команды локализации.

mod language_repository;
pub(crate) mod localization_commands;

pub(crate) use self::language_repository::LanguageRepository;
