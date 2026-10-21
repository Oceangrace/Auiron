use super::live_fields::LiveFields;
use super::selection_resolver::SelectionResolver;
use crate::core::game::GameSnapshot;
use crate::integrations::aion2::catalog::CatalogStore;
use crate::integrations::aion2::catalog::ZoneCatalog;
use crate::modules::interfaces::evaluation::FormulaDefinition;
use crate::modules::interfaces::evaluation::PreviewText;
use serde_json::Value;
use std::collections::HashMap;

pub(super) struct LiveInputs<'a> {
    game: &'a GameSnapshot,
    resolver: SelectionResolver<'a>,
    fields: HashMap<String, Value>,
    text: PreviewText,
    formulas: &'a [FormulaDefinition],
}

impl<'a> LiveInputs<'a> {
    pub(super) fn new(
        game: &'a GameSnapshot,
        catalog: &'a CatalogStore,
        zones: &ZoneCatalog,
        text: PreviewText,
        formulas: &'a [FormulaDefinition],
    ) -> Self {
        let mut fields = LiveFields::collect(game, zones);

        for prefix in ["target", "targetTarget"] {
            if !fields.contains_key(&format!("{prefix}.name"))
                && let Some(name) = fields
                    .get(&format!("{prefix}.npcId"))
                    .and_then(Value::as_u64)
                    .and_then(|id| u32::try_from(id).ok())
                    .and_then(|id| catalog.npc_name(id, text.locale()))
            {
                fields.insert(format!("{prefix}.name"), serde_json::json!(name));
            }
        }

        for prefix in ["player", "target", "targetTarget", "encounter"] {
            if let Some(name) = fields
                .get(&format!("{prefix}.cast.skillId"))
                .and_then(Value::as_u64)
                .and_then(|id| u32::try_from(id).ok())
                .and_then(|id| catalog.skill_name(id, text.locale()))
            {
                fields.insert(format!("{prefix}.cast.skillName"), serde_json::json!(name));
            }
        }

        Self {
            game,
            resolver: SelectionResolver::new(catalog),
            fields,
            text,
            formulas,
        }
    }

    pub(super) fn with_formulas(&self, formulas: &'a [FormulaDefinition]) -> Self {
        Self {
            formulas,
            ..self.with_fields(self.fields.clone())
        }
    }

    pub(super) fn with_fields(&self, fields: HashMap<String, Value>) -> Self {
        Self {
            game: self.game,
            resolver: self.resolver.clone(),
            fields,
            text: self.text.clone(),
            formulas: self.formulas,
        }
    }

    pub(super) fn game(&self) -> &GameSnapshot {
        self.game
    }

    pub(super) fn resolver(&self) -> &SelectionResolver<'_> {
        &self.resolver
    }

    pub(super) fn fields(&self) -> &HashMap<String, Value> {
        &self.fields
    }

    pub(super) fn text(&self) -> &PreviewText {
        &self.text
    }

    pub(super) fn formulas(&self) -> &[FormulaDefinition] {
        self.formulas
    }
}
