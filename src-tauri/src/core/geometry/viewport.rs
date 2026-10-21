use serde::Serialize;

#[derive(Clone, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Viewport {
    pub(crate) width: f64,
    pub(crate) height: f64,
}
