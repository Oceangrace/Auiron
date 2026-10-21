use crate::modules::interfaces::document::DocumentFormat;
use crate::modules::interfaces::document::InterfaceDocument;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct InterfaceWorkspace {
    #[schemars(range(min = DocumentFormat::VERSION, max = DocumentFormat::VERSION))]
    pub(in crate::modules::interfaces) format_version: u32,
    pub(in crate::modules::interfaces) revision: u64,
    #[schemars(length(max = 100))]
    pub(in crate::modules::interfaces) documents: Vec<InterfaceDocument>,
}

impl InterfaceWorkspace {
    pub(crate) fn empty() -> Self {
        Self {
            format_version: DocumentFormat::VERSION,
            revision: 0,
            documents: Vec::new(),
        }
    }
}
