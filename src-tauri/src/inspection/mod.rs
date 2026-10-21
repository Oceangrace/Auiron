//! Headless verification using production capture, state and interface evaluation.

mod input_markers;
mod journal_inspection;
mod live_inspection;
mod replay_inspection;
mod widget_replay_report;

pub(crate) use self::journal_inspection::JournalInspection;
pub(crate) use self::live_inspection::LiveInspection;
pub(crate) use self::replay_inspection::ReplayInspection;
