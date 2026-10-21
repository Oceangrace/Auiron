use super::DecoderKind;
use super::MessageDirection;
use super::OpcodeSupport;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct OpcodeDefinition {
    opcode: u16,
    name: String,
    direction: MessageDirection,
    support: OpcodeSupport,
}

impl OpcodeDefinition {
    pub(crate) fn opcode(&self) -> u16 {
        self.opcode
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn decoder(&self) -> Option<DecoderKind> {
        match &self.support {
            OpcodeSupport::Supported { decoder } | OpcodeSupport::Partial { decoder, .. } => {
                Some(*decoder)
            }

            OpcodeSupport::Unsupported { .. } => None,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.name.is_empty()
            || !self
                .name
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
        {
            return Err("Invalid protocol message name".into());
        }

        if let OpcodeSupport::Unsupported { reason } | OpcodeSupport::Partial { reason, .. } =
            &self.support
            && reason.trim().is_empty()
        {
            return Err("Incomplete messages require a reason".into());
        }

        Ok(())
    }
}
