//! Адаптеры системных API Windows.

mod github_release;
mod http_handle;
mod localization;
mod speech;
mod startup_dialog;
mod windows_icon_transport;
mod windows_releases;

pub(crate) use self::localization::WindowsTextMatcher;
pub(crate) use self::speech::WindowsSpeechEngine;
pub(crate) use self::startup_dialog::StartupDialog;
pub(crate) use self::windows_icon_transport::WindowsIconTransport;
pub(crate) use self::windows_releases::WindowsReleases;
