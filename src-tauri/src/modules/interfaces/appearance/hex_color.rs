use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub(crate) struct HexColor {
    value: String,
}

impl HexColor {
    pub(crate) fn as_str(&self) -> &str {
        &self.value
    }
}

impl TryFrom<String> for HexColor {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let valid = value.strip_prefix('#').is_some_and(|digits| {
            matches!(digits.len(), 6 | 8) && digits.bytes().all(|byte| byte.is_ascii_hexdigit())
        });

        if !valid {
            return Err("Invalid HEX color".into());
        }

        Ok(Self { value })
    }
}

impl From<HexColor> for String {
    fn from(color: HexColor) -> Self {
        color.value
    }
}

impl schemars::JsonSchema for HexColor {
    fn schema_name() -> Cow<'static, str> {
        "HexColor".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type": "string", "pattern": "^#[0-9a-fA-F]{6}([0-9a-fA-F]{2})?$"})
    }
}
