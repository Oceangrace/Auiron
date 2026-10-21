use super::DecoderKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub(crate) enum OpcodeSupport {
    Supported {
        decoder: DecoderKind,
    },
    Partial {
        decoder: DecoderKind,
        reason: String,
    },
    Unsupported {
        reason: String,
    },
}
