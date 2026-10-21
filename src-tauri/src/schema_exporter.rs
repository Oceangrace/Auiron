use crate::config::ApplicationConfiguration;
use crate::config::ApplicationDefinition;
use crate::integrations::aion2::catalog::CatalogManifest;
use crate::integrations::aion2::catalog::NpcCatalog;
use crate::integrations::aion2::catalog::RuntimeCatalog;
use crate::integrations::aion2::catalog::SkillNameCatalog;
use crate::integrations::aion2::protocol::OpcodeTable;
use crate::modules::combat_journal::JournalPreferences;
use crate::modules::interfaces::DocumentFormat;
use crate::modules::interfaces::FormulaDefinition;
use crate::modules::interfaces::FormulaOperation;
use crate::modules::interfaces::InterfaceDocument;
use crate::modules::interfaces::InterfaceWorkspace;
use crate::modules::interfaces::TriggerGroup;
use schemars::JsonSchema;
use serde_json::{Value, json};
use std::path::Path;

pub(crate) struct SchemaExporter;

impl SchemaExporter {
    pub(crate) fn export(check: bool) -> Result<(), String> {
        let configuration = ApplicationConfiguration::load()?;

        configuration.protocol()?;
        configuration.npc_index()?;
        configuration.runtime_catalog()?;

        let manifest = CatalogManifest::parse(configuration.manifest())?;
        let skill_names = include_bytes!(env!("AURION_CATALOG_SKILL_NAMES"));

        manifest.verify(configuration.catalog().skill_names(), skill_names)?;
        SkillNameCatalog::parse(
            skill_names,
            configuration.catalog().client_build(),
            manifest.version(),
        )?;

        let names = configuration.schemas();

        let directory = configuration.project_root().join(names.directory());

        if !check {
            std::fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
        }

        Self::document::<JournalPreferences>(&directory, names.journal(), check)?;
        Self::document::<ApplicationDefinition>(&directory, names.application(), check)?;
        Self::document::<InterfaceDocument>(&directory, names.document(), check)?;
        Self::document::<InterfaceWorkspace>(&directory, names.workspace(), check)?;
        Self::document::<TriggerGroup>(&directory, names.triggers(), check)?;
        Self::document::<FormulaDefinition>(&directory, names.formula(), check)?;
        Self::document::<OpcodeTable>(&directory, names.opcodes(), check)?;
        Self::document::<NpcCatalog>(&directory, names.npc_catalog(), check)?;
        Self::document::<SkillNameCatalog>(&directory, names.skill_names(), check)?;
        Self::document::<RuntimeCatalog>(&directory, names.runtime_catalog(), check)
    }

    fn document<T: JsonSchema>(directory: &Path, name: &Path, check: bool) -> Result<(), String> {
        let mut schema =
            serde_json::to_value(schemars::schema_for!(T)).map_err(|error| error.to_string())?;

        schema["$id"] = json!(format!(
            "urn:aurion:{}:v{}",
            T::schema_name(),
            DocumentFormat::VERSION
        ));
        Self::arity(&mut schema)?;

        let path = directory.join(name);

        if check {
            let saved: Value = serde_json::from_slice(
                &std::fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?,
            )
            .map_err(|error| error.to_string())?;

            if saved != schema {
                return Err(format!(
                    "Schema is out of date: {}. Run npm run schema",
                    path.display()
                ));
            }

            return Ok(());
        }

        let text = serde_json::to_string_pretty(&schema).map_err(|error| error.to_string())?;

        std::fs::write(&path, format!("{text}\n")).map_err(|error| error.to_string())
    }

    fn arity(schema: &mut Value) -> Result<(), String> {
        let Some(operations) = schema["$defs"]["FormulaOperation"]["enum"].as_array() else {
            return Ok(());
        };

        let mut constraints = Vec::new();

        for operation in operations {
            let parsed: FormulaOperation =
                serde_json::from_value(operation.clone()).map_err(|error| error.to_string())?;

            let (min, max) = parsed.arity();

            constraints.push(json!({
                "if": {"properties": {"operation": {"const": operation}}, "required": ["operation"]},
                "then": {"properties": {"operands": {"minItems": min, "maxItems": max}}}
            }));
        }

        let variants = schema["$defs"]["FormulaExpression"]["oneOf"]
            .as_array_mut()
            .ok_or("Formula expression schema is missing")?;

        let operation = variants
            .iter_mut()
            .find(|variant| variant["properties"]["kind"]["const"] == "operation")
            .ok_or("Formula operation schema is missing")?;

        operation["allOf"] = constraints.into();

        Ok(())
    }
}
