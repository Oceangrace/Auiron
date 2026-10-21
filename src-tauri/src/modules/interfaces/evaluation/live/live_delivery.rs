use super::live_frame::LiveFrame;
use super::live_update::LiveUpdate;
use serde_json::Value;

#[derive(Default)]
pub(crate) struct LiveDelivery {
    revision: u64,
    previous: Option<Value>,
}

impl LiveDelivery {
    pub(crate) fn deliver(
        &mut self,
        frame: LiveFrame,
        revision: Option<u64>,
    ) -> Result<LiveUpdate, String> {
        let mut value = serde_json::to_value(&frame).map_err(|error| error.to_string())?;

        // The snapshot time changes every publish; it alone is not a new frame.
        if let Some(object) = value.as_object_mut() {
            object.remove("atUs");
        }

        if self.previous.as_ref() != Some(&value) {
            self.revision = self
                .revision
                .checked_add(1)
                .ok_or("Live delivery revision exhausted")?;
            self.previous = Some(value);
        }

        let frame = if revision == Some(self.revision) {
            None
        } else {
            Some(frame)
        };

        Ok(LiveUpdate::new(self.revision, frame))
    }
}
