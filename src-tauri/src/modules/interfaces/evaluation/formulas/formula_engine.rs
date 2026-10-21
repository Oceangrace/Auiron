use super::FormulaConstant;
use super::FormulaDefinition;
use super::FormulaExpression;
use super::FormulaOperation;
use super::FormulaValueKind;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};

pub(crate) struct FormulaEngine;

impl FormulaEngine {
    pub(crate) fn validate(formulas: &[FormulaDefinition]) -> Result<(), String> {
        if formulas.len() > 32 {
            return Err("Up to 32 formulas".into());
        }

        let mut ids = HashSet::new();

        for formula in formulas {
            if formula.id.is_empty()
                || formula.id.len() > 100
                || formula.name.trim().is_empty()
                || formula.name.chars().count() > 128
                || !ids.insert(&formula.id)
            {
                return Err("Invalid formula".into());
            }

            Self::validate_node(&formula.expression, 0, &mut 0)?;
        }

        let mut visited = HashSet::new();

        for formula in formulas {
            Self::acyclic(
                &formula.expression,
                formulas,
                &mut vec![formula.id.clone()],
                &mut visited,
            )?;
        }

        Ok(())
    }

    fn validate_node(
        node: &FormulaExpression,
        depth: usize,
        count: &mut usize,
    ) -> Result<(), String> {
        *count += 1;

        if depth > 5 || *count > 128 {
            return Err("Formula is too complex".into());
        }

        match node {
            FormulaExpression::Constant {
                value: FormulaConstant::Number(value),
            } if !value.as_f64().is_some_and(f64::is_finite) => {
                return Err("Invalid formula node".into());
            }
            FormulaExpression::Field { path }
                if !Self::path(path) || path.starts_with("formula.") =>
            {
                return Err("Invalid formula node".into());
            }
            FormulaExpression::Formula { formula_id } if !Self::path(formula_id) => {
                return Err("Invalid formula node".into());
            }
            FormulaExpression::Operation {
                operation,
                operands,
            } => {
                let (min, max) = operation.arity();

                if !(min..=max).contains(&operands.len()) {
                    return Err("Invalid formula argument count".into());
                }

                for operand in operands {
                    Self::validate_node(operand, depth + 1, count)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn path(value: &str) -> bool {
        !value.is_empty() && value.len() <= 128
    }

    fn acyclic(
        node: &FormulaExpression,
        formulas: &[FormulaDefinition],
        stack: &mut Vec<String>,
        visited: &mut HashSet<String>,
    ) -> Result<(), String> {
        match node {
            FormulaExpression::Formula { formula_id } => {
                if visited.contains(formula_id) {
                    return Ok(());
                }

                if stack.contains(formula_id) {
                    return Err("Formula cycle".into());
                }

                let formula = formulas
                    .iter()
                    .find(|formula| formula.id == *formula_id)
                    .ok_or("Reference to a missing formula")?;

                stack.push(formula_id.clone());
                Self::acyclic(&formula.expression, formulas, stack, visited)?;
                visited.insert(formula_id.clone());
                stack.pop();
            }
            FormulaExpression::Operation { operands, .. } => {
                for operand in operands {
                    Self::acyclic(operand, formulas, stack, visited)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub(crate) fn fields(
        formulas: &[FormulaDefinition],
        source: &HashMap<String, Value>,
    ) -> HashMap<String, Value> {
        let mut fields = source.clone();

        for formula in formulas {
            let id = &formula.id;

            let value = Self::formula(id, formulas, source, &mut Vec::new(), &mut 512);

            fields.insert(format!("formula.{id}"), value.unwrap_or(Value::Null));
            fields.insert(format!("kind.formula.{id}"), json!(formula.kind));
        }

        fields
    }

    fn formula(
        id: &str,
        formulas: &[FormulaDefinition],
        fields: &HashMap<String, Value>,
        stack: &mut Vec<String>,
        budget: &mut usize,
    ) -> Option<Value> {
        if stack.len() >= 32 || stack.iter().any(|value| value == id) {
            return None;
        }

        let formula = formulas.iter().find(|formula| formula.id == id)?;

        stack.push(id.to_owned());
        let value = Self::node(&formula.expression, formulas, fields, stack, budget);

        stack.pop();
        let value = value?;

        if (formula.kind == FormulaValueKind::Boolean && value.is_boolean())
            || (formula.kind != FormulaValueKind::Boolean && value.is_number())
        {
            return Some(value);
        }

        None
    }

    fn node(
        node: &FormulaExpression,
        formulas: &[FormulaDefinition],
        fields: &HashMap<String, Value>,
        stack: &mut Vec<String>,
        budget: &mut usize,
    ) -> Option<Value> {
        *budget = budget.checked_sub(1)?;

        match node {
            FormulaExpression::Constant { value } => Some(match value {
                FormulaConstant::Boolean(value) => json!(value),
                FormulaConstant::Number(value) => json!(value),
            }),
            FormulaExpression::Field { path } => fields
                .get(path)
                .filter(|value| value.is_number() || value.is_boolean())
                .cloned(),
            FormulaExpression::Formula { formula_id } => {
                Self::formula(formula_id, formulas, fields, stack, budget)
            }
            FormulaExpression::Operation {
                operation,
                operands,
            } => {
                let values: Vec<Option<Value>> = operands
                    .iter()
                    .map(|operand| Self::node(operand, formulas, fields, stack, budget))
                    .collect();

                Self::operation(*operation, &values)
            }
        }
    }

    fn operation(operation: FormulaOperation, values: &[Option<Value>]) -> Option<Value> {
        if operation == FormulaOperation::Coalesce {
            return values
                .iter()
                .flatten()
                .find(|value| !value.is_null())
                .cloned();
        }

        if matches!(
            operation,
            FormulaOperation::All | FormulaOperation::Any | FormulaOperation::Not
        ) {
            if values.iter().flatten().any(|value| !value.is_boolean()) {
                return None;
            }

            let decisive = operation == FormulaOperation::Any;

            if values
                .iter()
                .flatten()
                .any(|value| value.as_bool() == Some(decisive))
            {
                return Some(json!(if operation == FormulaOperation::Not {
                    !decisive
                } else {
                    decisive
                }));
            }

            if values.iter().any(Option::is_none) {
                return None;
            }

            return Some(json!(if operation == FormulaOperation::Not {
                decisive
            } else {
                !decisive
            }));
        }

        let values: Vec<&Value> = values.iter().map(Option::as_ref).collect::<Option<_>>()?;

        if matches!(
            operation,
            FormulaOperation::Equal | FormulaOperation::NotEqual
        ) {
            let (left, right) = (*values.first()?, *values.get(1)?);

            if left.is_number() != right.is_number() || left.is_boolean() != right.is_boolean() {
                return None;
            }

            return Some(json!(
                (left == right) == (operation == FormulaOperation::Equal)
            ));
        }

        let n: Vec<f64> = values
            .iter()
            .map(|value| value.as_f64())
            .collect::<Option<_>>()?;

        let first = *n.first()?;

        if matches!(
            operation,
            FormulaOperation::Less
                | FormulaOperation::LessOrEqual
                | FormulaOperation::Greater
                | FormulaOperation::GreaterOrEqual
        ) {
            let second = *n.get(1)?;

            return Some(json!(match operation {
                FormulaOperation::Less => first < second,
                FormulaOperation::LessOrEqual => first <= second,
                FormulaOperation::Greater => first > second,
                FormulaOperation::GreaterOrEqual => first >= second,
                _ => return None,
            }));
        }

        let value = match operation {
            FormulaOperation::Add => n.iter().sum(),
            FormulaOperation::Multiply => n.iter().product(),
            FormulaOperation::Subtract => first - n.get(1)?,
            FormulaOperation::Divide if *n.get(1)? != 0.0 => first / n.get(1)?,
            FormulaOperation::Minimum => n.iter().copied().fold(f64::INFINITY, f64::min),
            FormulaOperation::Maximum => n.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            FormulaOperation::Round => (first + 0.5).floor(),
            FormulaOperation::Absolute => first.abs(),
            FormulaOperation::Clamp if n.get(1)? <= n.get(2)? => {
                first.clamp(*n.get(1)?, *n.get(2)?)
            }
            _ => return None,
        };

        value.is_finite().then(|| json!(value))
    }
}
