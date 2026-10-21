use crate::core::geometry::OverlayRegion;
use std::sync::Arc;

#[derive(Default)]
pub(crate) struct OverlayInput {
    regions: Arc<[OverlayRegion]>,
    dragging: bool,
}

impl OverlayInput {
    pub(super) fn set_regions(&mut self, regions: Vec<OverlayRegion>) -> Result<(), String> {
        if regions.len() > 10000 || regions.iter().any(|region| !region.is_valid()) {
            return Err("Invalid overlay regions".into());
        }

        self.regions = regions.into();

        Ok(())
    }

    pub(super) fn set_dragging(&mut self, dragging: bool) {
        self.dragging = dragging;
    }

    pub(super) fn snapshot(&self) -> (bool, Arc<[OverlayRegion]>) {
        (self.dragging, Arc::clone(&self.regions))
    }
}
