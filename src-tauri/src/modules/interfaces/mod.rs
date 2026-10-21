//! Нативная часть редактора. Внешний доступ — через сервис, репозиторий и команды.

mod appearance;
mod audio;
mod document;
mod evaluation;
mod exchange;
pub(crate) mod interface_commands;
mod interface_service;
mod layout;
mod persistence;
mod rules;
mod validation;

pub(crate) use self::audio::SpeechService;
pub(crate) use self::audio::speech_commands;
#[cfg(feature = "schema")]
pub(crate) use self::document::DocumentFormat;
#[cfg(feature = "schema")]
pub(crate) use self::document::InterfaceDocument;
#[cfg(feature = "schema")]
pub(crate) use self::document::InterfaceWorkspace;
#[cfg(feature = "schema")]
pub(crate) use self::evaluation::FormulaDefinition;
#[cfg(feature = "schema")]
pub(crate) use self::evaluation::FormulaOperation;
pub(crate) use self::evaluation::LiveFrame;
pub(crate) use self::evaluation::PreviewText;
pub(crate) use self::evaluation::TriggerState;
pub(crate) use self::interface_service::InterfaceService;
pub(crate) use self::persistence::InterfaceRepository;
#[cfg(feature = "schema")]
pub(crate) use self::rules::TriggerGroup;
