use crate::core::game::CharacterClass;
use crate::modules::interfaces::appearance::WidgetAppearance;
use crate::modules::interfaces::appearance::WidgetStyle;
use crate::modules::interfaces::audio::SoundCue;
use crate::modules::interfaces::document::WidgetNode;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PreviewVisual {
    pub(in crate::modules::interfaces) class_name: Option<CharacterClass>,
    pub(in crate::modules::interfaces) simulating: bool,
    pub(in crate::modules::interfaces) animation_value: Option<serde_json::Value>,
    pub(in crate::modules::interfaces) active: Option<bool>,
    pub(in crate::modules::interfaces) visible: Option<bool>,
    pub(in crate::modules::interfaces) remaining: Option<f64>,
    pub(in crate::modules::interfaces) duration: Option<f64>,
    pub(in crate::modules::interfaces) stacks: Option<f64>,
    pub(in crate::modules::interfaces) known_progress: bool,
    pub(in crate::modules::interfaces) progress_identity: Option<String>,
    pub(in crate::modules::interfaces) healing_prediction: Option<super::HealingPrediction>,
    pub(in crate::modules::interfaces) ticks: Vec<f64>,
    pub(in crate::modules::interfaces) animation_active: bool,
    pub(in crate::modules::interfaces) sound: SoundCue,
    pub(in crate::modules::interfaces) speech_text: String,
    pub(in crate::modules::interfaces) offset_x: f64,
    pub(in crate::modules::interfaces) offset_y: f64,
    pub(in crate::modules::interfaces) child_visuals: HashMap<String, PreviewVisual>,
    pub(in crate::modules::interfaces) instances: Vec<PreviewInstance>,
    pub(in crate::modules::interfaces) appearance: WidgetAppearance,
    pub(in crate::modules::interfaces) style: WidgetStyle,
}

impl PreviewVisual {
    pub(crate) fn visible_instances(&self) -> usize {
        if self.instances.is_empty() {
            return usize::from(self.visible == Some(true));
        }

        self.instances
            .iter()
            .map(|instance| instance.visual.visible_instances())
            .sum()
    }

    pub(crate) fn remaining(&self) -> Option<f64> {
        self.remaining
    }

    pub(crate) fn from_node(node: &WidgetNode) -> Self {
        Self {
            class_name: None,
            simulating: false,
            animation_value: None,
            appearance: node.appearance.clone(),
            style: node.style.clone(),
            active: None,
            visible: None,
            remaining: None,
            duration: None,
            stacks: None,
            known_progress: true,
            progress_identity: None,
            healing_prediction: None,
            ticks: Vec::new(),
            animation_active: true,
            sound: SoundCue::None,
            speech_text: String::new(),
            offset_x: 0.0,
            offset_y: 0.0,
            instances: Vec::new(),
            child_visuals: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, schemars::JsonSchema)]
pub(crate) struct PreviewInstance {
    pub(in crate::modules::interfaces) key: String,
    pub(in crate::modules::interfaces) visual: Box<PreviewVisual>,
}
