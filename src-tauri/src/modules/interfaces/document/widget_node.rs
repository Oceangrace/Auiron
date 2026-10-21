use crate::modules::interfaces::appearance::WidgetAppearance;
use crate::modules::interfaces::appearance::WidgetStyle;
use crate::modules::interfaces::audio::WidgetSounds;
use crate::modules::interfaces::document::WidgetKind;
use crate::modules::interfaces::layout::GroupDataBinding;
use crate::modules::interfaces::layout::GroupHeaderButton;
use crate::modules::interfaces::layout::GroupSettings;
use crate::modules::interfaces::layout::WidgetAnchors;
use crate::modules::interfaces::layout::WidgetLayout;
use crate::modules::interfaces::rules::CatalogSelection;
use crate::modules::interfaces::rules::LoadRules;
use crate::modules::interfaces::rules::TriggerDefinition;
use crate::modules::interfaces::rules::TriggerGroup;
use crate::modules::interfaces::rules::WidgetCondition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct WidgetNode {
    #[serde(default)]
    pub(in crate::modules::interfaces) rule_selections: HashMap<String, Vec<CatalogSelection>>,
    #[serde(default)]
    pub(in crate::modules::interfaces) image_selection: Option<CatalogSelection>,
    #[serde(default)]
    pub(in crate::modules::interfaces) rules: Option<TriggerGroup>,
    #[serde(default)]
    pub(in crate::modules::interfaces) binding: Option<GroupDataBinding>,
    #[serde(default)]
    #[schemars(length(max = 6))]
    pub(in crate::modules::interfaces) header_buttons: Vec<GroupHeaderButton>,
    #[serde(default)]
    pub(in crate::modules::interfaces) load: LoadRules,
    #[serde(default)]
    pub(in crate::modules::interfaces) sounds: WidgetSounds,
    #[serde(default)]
    #[schemars(length(min = 1, max = 64))]
    pub(in crate::modules::interfaces) entity_id: Option<String>,
    #[schemars(length(min = 1, max = 100))]
    pub(in crate::modules::interfaces) id: String,
    #[schemars(length(min = 1, max = 128))]
    pub(in crate::modules::interfaces) name: String,
    pub(in crate::modules::interfaces) kind: WidgetKind,
    pub(in crate::modules::interfaces) enabled: bool,
    #[serde(default)]
    pub(in crate::modules::interfaces) anchors: WidgetAnchors,
    pub(in crate::modules::interfaces) layout: WidgetLayout,
    pub(in crate::modules::interfaces) appearance: WidgetAppearance,
    #[serde(default)]
    pub(in crate::modules::interfaces) style: WidgetStyle,
    #[serde(default)]
    pub(in crate::modules::interfaces) group: GroupSettings,
    #[serde(default)]
    #[schemars(length(max = 32))]
    pub(in crate::modules::interfaces) conditions: Vec<WidgetCondition>,
    pub(in crate::modules::interfaces) trigger: Option<TriggerDefinition>,
    #[schemars(length(max = 2000))]
    pub(in crate::modules::interfaces) children: Vec<WidgetNode>,
}

impl WidgetNode {
    pub(in crate::modules::interfaces) fn scale(&self) -> f64 {
        if matches!(self.kind, WidgetKind::Group | WidgetKind::DynamicGroup) {
            return self.group.scale / 100.0;
        }

        1.0
    }
}
