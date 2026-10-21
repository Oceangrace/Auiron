use serde::Serialize;

#[derive(Serialize, schemars::JsonSchema)]
pub(crate) struct LocaleOption {
    pub(crate) value: String,
    pub(crate) label: String,
}
