use super::ComparisonOperator;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum ValueOperator {
    #[serde(rename = "eq")]
    Equal,
    #[serde(rename = "ne")]
    NotEqual,
    #[serde(rename = "lt")]
    Less,
    #[serde(rename = "lte")]
    LessEqual,
    #[serde(rename = "gt")]
    Greater,
    #[serde(rename = "gte")]
    GreaterEqual,
}

impl From<ValueOperator> for ComparisonOperator {
    fn from(value: ValueOperator) -> Self {
        match value {
            ValueOperator::Equal => Self::Equal,
            ValueOperator::NotEqual => Self::NotEqual,
            ValueOperator::Less => Self::Less,
            ValueOperator::LessEqual => Self::LessEqual,
            ValueOperator::Greater => Self::Greater,
            ValueOperator::GreaterEqual => Self::GreaterEqual,
        }
    }
}
