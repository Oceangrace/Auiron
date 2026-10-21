use crate::core::diagnostics::PerfLog;
use crate::core::game::GameEventType;
use crate::core::game::SourceFrame;
use crate::core::geometry::Viewport;
use crate::integrations::aion2::catalog::CatalogPage;
use crate::integrations::aion2::catalog::CatalogRecord;
use crate::integrations::aion2::catalog::CatalogScope;
use crate::integrations::aion2::catalog::CatalogSearch;
use crate::integrations::aion2::catalog::CatalogStore;
use crate::integrations::aion2::catalog::FieldBossCatalog;
use crate::integrations::aion2::catalog::ZoneCatalog;
use crate::modules::interfaces::InterfaceRepository;
use crate::modules::interfaces::document::DocumentFormat;
use crate::modules::interfaces::document::InterfaceDocument;
use crate::modules::interfaces::document::InterfaceSnapshot;
use crate::modules::interfaces::document::InterfaceWorkspace;
use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::DocumentLocalizer;
use crate::modules::interfaces::evaluation::LiveDelivery;
use crate::modules::interfaces::evaluation::LiveEvaluator;
use crate::modules::interfaces::evaluation::LiveFrame;
use crate::modules::interfaces::evaluation::LiveUpdate;
use crate::modules::interfaces::evaluation::PreviewContext;
use crate::modules::interfaces::evaluation::PreviewEvaluator;
use crate::modules::interfaces::evaluation::PreviewText;
use crate::modules::interfaces::evaluation::PreviewVisual;
use crate::modules::interfaces::exchange::ImportPreview;
use crate::modules::interfaces::exchange::InterfaceCodec;
use crate::modules::interfaces::rules::CatalogSelection;
use crate::modules::interfaces::rules::ObservationKind;
use crate::modules::interfaces::rules::TriggerRule;
use crate::modules::interfaces::rules::TriggerSubject;
use crate::modules::interfaces::validation::InterfaceValidator;
use crate::modules::interfaces::validation::WidgetVisualValidator;
use std::collections::HashMap;
use std::time::Instant;

pub(crate) struct InterfaceService {
    repository: InterfaceRepository,
    catalog: CatalogStore,
    snapshot: InterfaceSnapshot,
    live: LiveEvaluator,
    delivery: LiveDelivery,
    zones: ZoneCatalog,
    perf: PerfLog,
}

impl InterfaceService {
    pub(crate) fn new(
        repository: InterfaceRepository,
        catalog: CatalogStore,
        viewport: Viewport,
        zones: ZoneCatalog,
    ) -> Self {
        Self {
            repository,
            zones,
            live: LiveEvaluator::default(),
            delivery: LiveDelivery::default(),
            perf: PerfLog::new("interfaces_live"),
            catalog,
            snapshot: InterfaceSnapshot {
                workspace: InterfaceWorkspace::empty(),
                viewport,
                load_error: None,
            },
        }
    }

    pub(crate) fn start(&mut self) {
        match self.repository.load() {
            Ok(workspace) => self.snapshot.workspace = workspace,
            Err(error) => self.snapshot.load_error = Some(error),
        }
    }

    pub(crate) fn validate_loaded(&self) -> Result<(), String> {
        if let Some(error) = &self.snapshot.load_error {
            return Err(error.clone());
        }

        self.validate(&self.snapshot.workspace)
    }

    pub(crate) fn read(&self) -> InterfaceSnapshot {
        self.snapshot.clone()
    }

    pub(crate) fn search(&self, request: CatalogSearch) -> Result<CatalogPage, String> {
        self.catalog.search(request)
    }

    pub(crate) fn catalog_item(
        &self,
        scope: &CatalogScope,
        context: Option<&str>,
        key: &str,
        locale: &str,
    ) -> CatalogRecord {
        CatalogRecord {
            catalog_version: self.catalog.version().to_owned(),
            item: self.catalog.localized_item(scope, context, key, locale),
        }
    }

    pub(crate) fn live(
        &mut self,
        source: &SourceFrame,
        text: PreviewText,
    ) -> Result<LiveFrame, String> {
        self.live.evaluate(
            &self.snapshot.workspace,
            source,
            &self.catalog,
            &self.zones,
            text,
        )
    }

    pub(crate) fn live_update(
        &mut self,
        source: &SourceFrame,
        text: PreviewText,
        revision: Option<u64>,
    ) -> Result<LiveUpdate, String> {
        let started = Instant::now();

        let frame = self.live(source, text)?;

        let update = self.delivery.deliver(frame, revision)?;

        let bytes = (self.perf.enabled() && update.has_frame())
            .then(|| serde_json::to_vec(&update).map_or(0, |bytes| bytes.len()));

        self.perf.record(started.elapsed(), bytes);

        Ok(update)
    }

    pub(crate) fn preview(
        &self,
        workspace: &InterfaceWorkspace,
        context: &PreviewContext,
    ) -> Result<HashMap<String, PreviewVisual>, String> {
        InterfaceValidator::new().validate(workspace)?;
        WidgetVisualValidator::context(context)?;
        let mut context = context.clone();

        for boss in FieldBossCatalog::all() {
            if let Some(name) = self.catalog.npc_name(boss.npc_id, context.text().locale()) {
                context.fields.insert(
                    format!("worldBoss.{}.name", boss.id),
                    serde_json::json!(name),
                );
            }
        }

        let mut result = HashMap::new();

        for document in &workspace.documents {
            let localized = DocumentLocalizer::apply(document, context.text().locale());
            let document = localized.as_ref();

            result.extend(PreviewEvaluator::evaluate(
                &document.roots,
                &context,
                &document.formulas,
            )?);
        }

        Ok(result)
    }

    pub(crate) fn validate(&self, workspace: &InterfaceWorkspace) -> Result<(), String> {
        InterfaceValidator::new().validate(workspace)?;

        for document in &workspace.documents {
            self.validate_triggers(&document.roots)?;
        }

        Ok(())
    }

    pub(crate) fn inspect_import(&self, text: &str) -> Result<ImportPreview, String> {
        let preview = InterfaceCodec::decode(text)?;

        self.validate(&InterfaceWorkspace {
            format_version: DocumentFormat::VERSION,
            revision: 0,
            documents: vec![preview.document.clone()],
        })?;

        Ok(preview)
    }

    pub(crate) fn export(&self, document: &InterfaceDocument) -> Result<String, String> {
        self.validate(&InterfaceWorkspace {
            format_version: DocumentFormat::VERSION,
            revision: 0,
            documents: vec![document.clone()],
        })?;

        InterfaceCodec::encode(document)
    }

    pub(crate) fn save(
        &mut self,
        mut workspace: InterfaceWorkspace,
    ) -> Result<InterfaceSnapshot, String> {
        if self.snapshot.load_error.is_some() {
            return Err("Saving is disabled: the original file needs recovery".into());
        }

        if workspace.revision != self.snapshot.workspace.revision {
            return Err(
                "Settings changed in another window. Your edits remain in the open editor.".into(),
            );
        }

        self.validate(&workspace)?;
        let expected = workspace.revision;

        workspace.revision = expected.checked_add(1).ok_or("Revision overflow")?;
        self.repository.save(&workspace, &self.snapshot.workspace)?;
        self.snapshot.workspace = workspace;

        Ok(self.read())
    }

    fn selection_ids(&self, selection: &CatalogSelection) -> Result<Vec<u32>, String> {
        if selection.catalog_version != self.catalog.version() {
            return Err("Catalog changed: choose the variant again".into());
        }

        self.catalog
            .find(
                &selection.scope,
                selection.context_key.as_deref(),
                &selection.entry_key,
            )
            .map(|item| item.match_ids())
            .filter(|ids| !ids.is_empty())
            .ok_or("Catalog variant is unavailable".into())
    }

    fn selection_scope(rule: &TriggerRule, selection: &CatalogSelection) -> bool {
        let effect = match rule {
            TriggerRule::Effect { .. } => true,
            TriggerRule::Event { trigger, .. } => matches!(
                trigger.event,
                GameEventType::EffectApplied
                    | GameEventType::EffectUpdated
                    | GameEventType::EffectRemoved
            ),
            TriggerRule::Boss { .. } | TriggerRule::Value { .. } => return false,
        };

        effect
            == matches!(
                selection.scope,
                CatalogScope::Effects | CatalogScope::EnemyEffects
            )
    }

    fn validate_triggers(&self, nodes: &[WidgetNode]) -> Result<(), String> {
        for node in nodes {
            if let Some(selection) = &node.image_selection {
                self.selection_ids(selection)?;
            }

            for (id, selections) in &node.rule_selections {
                let rule = node
                    .rules
                    .as_ref()
                    .and_then(|group| group.rules.iter().find(|rule| rule.id() == id))
                    .ok_or("Catalog selection refers to a missing rule")?;

                let mut actual = Vec::new();

                for selection in selections {
                    if !Self::selection_scope(rule, selection) {
                        return Err(
                            "Rule catalog type does not match the selected observation".into()
                        );
                    }

                    actual.extend(self.selection_ids(selection)?);
                }

                let expected = rule.match_ids();

                if !selections.is_empty() && expected.iter().any(|id| !actual.contains(id)) {
                    return Err("Rule filter does not match the selected catalog variants".into());
                }
            }

            if let Some(trigger) = &node.trigger {
                let selection = &trigger.selection;

                if selection.catalog_version != self.catalog.version() {
                    return Err(format!(
                        "{}: catalog changed, select the entry again",
                        node.name
                    ));
                }

                let item = self
                    .catalog
                    .find(
                        &selection.scope,
                        selection.context_key.as_deref(),
                        &selection.entry_key,
                    )
                    .ok_or_else(|| {
                        format!("{}: catalog entry is no longer supported", node.name)
                    })?;

                if !item
                    .forms
                    .iter()
                    .any(|form| form.key == selection.choice_key && !form.match_ids.is_empty())
                {
                    return Err(format!("{}: selected form is not supported", node.name));
                }

                let scope_matches = match trigger.observation {
                    ObservationKind::EffectPresent => matches!(
                        selection.scope,
                        CatalogScope::Effects | CatalogScope::EnemyEffects
                    ),
                    ObservationKind::SkillUsed
                    | ObservationKind::SkillState
                    | ObservationKind::Cooldown
                    | ObservationKind::SkillReady
                    | ObservationKind::ComboAvailable => matches!(
                        selection.scope,
                        CatalogScope::Skills | CatalogScope::EnemySkills
                    ),
                };

                if !scope_matches {
                    return Err("Observation type does not match the selected catalog".into());
                }

                if trigger.subject == TriggerSubject::Enemy {
                    let actor = trigger.actor_key.as_deref().ok_or("No enemy selected")?;

                    if trigger.observation != ObservationKind::SkillUsed
                        || selection.scope != CatalogScope::EnemySkills
                        || selection.context_key.as_deref() != Some(actor)
                    {
                        return Err("Enemy observations currently support skill use only".into());
                    }
                } else if trigger.actor_key.is_some() {
                    return Err("Self observations must not have an enemy filter".into());
                }

                match trigger.observation {
                    ObservationKind::SkillUsed => {
                        let seconds = trigger
                            .display_seconds
                            .ok_or("Notification duration is missing")?;

                        if !seconds.is_finite() || !(0.1..=3600.0).contains(&seconds) {
                            return Err(
                                "Notification duration must be between 0.1 and 3600 seconds".into(),
                            );
                        }
                    }
                    ObservationKind::EffectPresent
                    | ObservationKind::SkillState
                    | ObservationKind::Cooldown
                    | ObservationKind::SkillReady
                    | ObservationKind::ComboAvailable => {
                        if trigger.display_seconds.is_some() {
                            return Err("Observed duration cannot be set manually".into());
                        }
                    }
                }
            }

            self.validate_triggers(&node.children)?;
        }

        Ok(())
    }
}
