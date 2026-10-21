use crate::modules::interfaces::document::InterfaceDocument;
use serde::Serialize;

#[derive(Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImportPreview {
    pub(in crate::modules::interfaces) document: InterfaceDocument,
}
