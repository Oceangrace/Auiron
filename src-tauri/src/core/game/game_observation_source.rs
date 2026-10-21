use super::SourceFrame;
use std::sync::Arc;

pub(crate) trait GameObservationSource: Send + Sync {
    fn frame(&self) -> Result<Arc<SourceFrame>, String>;
}
