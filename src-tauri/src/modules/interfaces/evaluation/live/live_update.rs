use super::live_frame::LiveFrame;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LiveUpdate {
    revision: u64,
    frame: Option<LiveFrame>,
}

impl LiveUpdate {
    pub(super) fn new(revision: u64, frame: Option<LiveFrame>) -> Self {
        Self { revision, frame }
    }

    pub(crate) fn has_frame(&self) -> bool {
        self.frame.is_some()
    }
}
