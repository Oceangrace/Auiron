use super::FormulaConstant;
use super::FormulaOperation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(
    tag = "kind",
    rename_all = "kebab-case",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum FormulaExpression {
    Constant {
        value: FormulaConstant,
    },
    Field {
        #[schemars(length(min = 1, max = 128))]
        path: String,
    },
    Formula {
        #[schemars(length(min = 1, max = 128))]
        formula_id: String,
    },
    Operation {
        operation: FormulaOperation,
        #[schemars(length(min = 1, max = 8))]
        operands: Vec<FormulaExpression>,
    },
}
