use crate::modules::interfaces::appearance::TextAlignment;
use crate::modules::interfaces::layout::Direction;
use crate::modules::interfaces::layout::GroupFlow;
use crate::modules::interfaces::layout::GroupGrow;
use crate::modules::interfaces::layout::GroupSort;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub(crate) struct GroupSettings {
    pub(in crate::modules::interfaces) fill_enabled: bool,
    pub(in crate::modules::interfaces) header_enabled: bool,
    pub(in crate::modules::interfaces) header_height: f64,
    pub(in crate::modules::interfaces) header_text: String,
    pub(in crate::modules::interfaces) header_color: String,
    pub(in crate::modules::interfaces) header_background: String,
    #[schemars(range(min = 25, max = 400))]
    pub(in crate::modules::interfaces) scale: f64,
    pub(in crate::modules::interfaces) padding: f64,
    pub(in crate::modules::interfaces) locked: bool,
    pub(in crate::modules::interfaces) flow: GroupFlow,
    pub(in crate::modules::interfaces) gap: f64,
    pub(in crate::modules::interfaces) columns: f64,
    pub(in crate::modules::interfaces) header_font_size: f64,
    pub(in crate::modules::interfaces) header_alignment: TextAlignment,
    pub(in crate::modules::interfaces) sort: GroupSort,
    pub(in crate::modules::interfaces) grow: GroupGrow,
    pub(in crate::modules::interfaces) limit: f64,
    pub(in crate::modules::interfaces) motion_ms: f64,
    pub(in crate::modules::interfaces) travel: Direction,
    pub(in crate::modules::interfaces) span_seconds: f64,
    pub(in crate::modules::interfaces) fill_opacity: Option<f64>,
    pub(in crate::modules::interfaces) header_opacity: Option<f64>,
}

impl Default for GroupSettings {
    fn default() -> Self {
        Self {
            fill_enabled: true,
            header_enabled: false,
            header_height: 28.0,
            header_text: "".to_owned(),
            header_color: "#d6b26e".to_owned(),
            header_background: "#141414".to_owned(),
            scale: 100.0,
            padding: 0.0,
            locked: false,
            flow: GroupFlow::Row,
            gap: 8.0,
            columns: 4.0,
            header_font_size: 14.0,
            header_alignment: TextAlignment::Left,
            sort: GroupSort::Manual,
            grow: GroupGrow::Forward,
            limit: 0.0,
            motion_ms: 0.0,
            travel: Direction::Left,
            span_seconds: 30.0,
            fill_opacity: None,
            header_opacity: None,
        }
    }
}
