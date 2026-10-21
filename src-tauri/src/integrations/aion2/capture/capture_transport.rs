use super::CaptureProcess;
use super::CaptureRead;
use crate::core::game::SourceStatus;

pub(crate) trait CaptureTransport {
    fn start(&mut self) -> Result<(), String>;
    fn poll(&mut self) -> Result<CaptureRead, String>;
    fn now_us(&self) -> u64;
    fn mode(&self) -> SourceStatus;
    fn server_ports(&self) -> &[u16];
    fn process(&self) -> Option<CaptureProcess> {
        None
    }

    fn stop(&mut self);
}
