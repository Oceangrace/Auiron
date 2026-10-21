mod application;
mod config;
mod core;
mod integrations;
mod modules;
mod runtime;
#[cfg(feature = "schema")]
mod schema_exporter;

use application::Application;
use std::path::Path;

pub fn run() {
    Application::new().start();
}

#[cfg(feature = "schema")]
pub fn export_schema(check: bool) -> Result<(), String> {
    schema_exporter::SchemaExporter::export(check)
}

pub fn inspect_traffic(
    path: &Path,
    server_port: u16,
    events_path: Option<&Path>,
) -> Result<String, String> {
    integrations::aion2::TrafficInspector::inspect(path, server_port, events_path)
}

pub fn probe_capture() -> Result<String, String> {
    integrations::aion2::capture::CaptureProbe::run()
}

mod inspection;

pub fn inspect_journal(recording: &Path, port: u16, output: &Path) -> Result<String, String> {
    inspection::JournalInspection::run(recording, port, output)
}

pub fn record_traffic(directory: &Path, seconds: u64) -> Result<String, String> {
    inspection::LiveInspection::record(directory, seconds)
}

pub fn record_marked_traffic(directory: &Path, seconds: u64) -> Result<String, String> {
    inspection::LiveInspection::record_with_markers(directory, seconds, true)
}

pub fn inspect_interfaces(recording: &Path, port: u16, workspace: &Path) -> Result<String, String> {
    inspection::ReplayInspection::run(recording, port, workspace)
}
