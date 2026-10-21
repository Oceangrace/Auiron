//! Independent combat journal: game projection and typed preferences.
mod journal_action;
mod journal_action_kind;
mod journal_appearance;
mod journal_archive;
mod journal_border_style;
pub(crate) mod journal_commands;
mod journal_effect;
mod journal_font;
mod journal_frame;
mod journal_header_button;
mod journal_metric;
mod journal_panel_layout;
mod journal_period;
mod journal_preferences;
mod journal_report;
mod journal_repository;
mod journal_resource;
mod journal_scope;
mod journal_unit;
mod journal_window;
pub(crate) use self::journal_archive::JournalArchive;
pub(crate) use self::journal_frame::JournalFrame;
pub(crate) use self::journal_report::JournalReport;

pub(crate) use self::journal_preferences::JournalPreferences;

pub(crate) use self::journal_repository::JournalRepository;
