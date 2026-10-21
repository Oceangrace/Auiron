use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum FormulaOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Minimum,
    Maximum,
    Round,
    Absolute,
    Clamp,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    All,
    Any,
    Not,
    Coalesce,
}

impl FormulaOperation {
    pub(crate) fn arity(self) -> (usize, usize) {
        match self {
            Self::Round | Self::Absolute | Self::Not => (1, 1),
            Self::Subtract
            | Self::Divide
            | Self::Equal
            | Self::NotEqual
            | Self::Less
            | Self::LessOrEqual
            | Self::Greater
            | Self::GreaterOrEqual => (2, 2),
            Self::Clamp => (3, 3),
            Self::Add
            | Self::Multiply
            | Self::Minimum
            | Self::Maximum
            | Self::All
            | Self::Any
            | Self::Coalesce => (2, 8),
        }
    }
}
