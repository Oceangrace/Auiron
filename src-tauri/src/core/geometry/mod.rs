//! Координаты, привязки, размеры экрана и интерактивные области.

mod overlay_anchor;
mod overlay_position;
mod overlay_region;
mod viewport;

pub(crate) use self::overlay_anchor::OverlayAnchor;
pub(crate) use self::overlay_position::OverlayPosition;
pub(crate) use self::overlay_region::OverlayRegion;
pub(crate) use self::viewport::Viewport;
