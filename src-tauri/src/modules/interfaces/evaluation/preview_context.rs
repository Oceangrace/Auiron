use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Default, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PreviewContext {
    #[serde(skip)]
    text: super::PreviewText,
    #[serde(default)]
    pub(in crate::modules::interfaces) simulating: bool,
    #[serde(default)]
    pub(in crate::modules::interfaces) fields: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub(in crate::modules::interfaces) widgets: HashMap<String, HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub(in crate::modules::interfaces) rows: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub(in crate::modules::interfaces) instances:
        HashMap<String, Vec<HashMap<String, serde_json::Value>>>,
    pub(in crate::modules::interfaces) active: Option<bool>,
    pub(in crate::modules::interfaces) remaining: Option<f64>,
    pub(in crate::modules::interfaces) stacks: Option<f64>,
    pub(in crate::modules::interfaces) progress: Option<f64>,
}

impl PreviewContext {
    pub(crate) fn set_text(&mut self, text: super::PreviewText) {
        self.text = text;
    }

    pub(crate) fn text(&self) -> &super::PreviewText {
        &self.text
    }
}
