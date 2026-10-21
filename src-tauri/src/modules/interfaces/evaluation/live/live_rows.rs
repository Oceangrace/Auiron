use super::live_evaluator::LiveEvaluator;
use super::live_fields::LiveFields;
use super::live_frame::LiveFrame;
use super::live_inputs::LiveInputs;
use super::live_row::LiveRow;
use super::trigger_state::TriggerState;
use super::world_boss_rows::WorldBossRows;
use crate::core::game::EntityId;
use crate::core::game::PartyMember;
use crate::core::game::SourceStatus;
use crate::modules::interfaces::document::WidgetKind;
use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::PreviewInstance;
use crate::modules::interfaces::evaluation::PreviewVisual;
use crate::modules::interfaces::layout::GroupDataSource;
use crate::modules::interfaces::layout::GroupFlow;
use serde_json::{Value, json};
use std::collections::HashMap;

pub(super) struct LiveRows;

impl LiveRows {
    pub(super) fn supported(node: &WidgetNode) -> bool {
        if node
            .binding
            .as_ref()
            .is_some_and(|binding| binding.source == GroupDataSource::SkillHistory)
        {
            return node.kind == WidgetKind::DynamicGroup
                && node.group.flow == GroupFlow::Timeline
                && node.children.iter().all(|child| {
                    child.kind == WidgetKind::Icon
                        && child.children.is_empty()
                        && child.binding.is_none()
                });
        }

        if node
            .binding
            .as_ref()
            .is_some_and(|binding| binding.source == GroupDataSource::Effects)
        {
            return node
                .binding
                .as_ref()
                .is_some_and(|binding| binding.effects.is_some())
                && node.group.flow != GroupFlow::Timeline;
        }

        node.binding.as_ref().is_some_and(|binding| {
            binding.source != GroupDataSource::Preview
                && (binding.source == GroupDataSource::PartyMembers || !binding.show_breakdown)
        }) && node.group.flow != GroupFlow::Timeline
            && node.children.iter().all(Self::unbound)
    }

    fn unbound(node: &WidgetNode) -> bool {
        node.binding.is_none() && node.children.iter().all(Self::unbound)
    }

    pub(super) fn populate(
        evaluator: &mut LiveEvaluator,
        node: &WidgetNode,
        inputs: &LiveInputs<'_>,
        frame: &mut LiveFrame,
    ) -> Result<(), String> {
        let game = inputs.game();

        let fresh = matches!(
            game.header().status(),
            SourceStatus::Live | SourceStatus::Replay
        );

        let rows = if !fresh {
            None
        } else if node
            .binding
            .as_ref()
            .is_some_and(|binding| binding.source == GroupDataSource::PartyMembers)
        {
            game.party().map(|roster| {
                roster
                    .members()
                    .iter()
                    .enumerate()
                    .map(|(index, member)| {
                        LiveRow::new(
                            format!("{}:{}", member.database_id(), member.slot()),
                            member.entity_id(),
                            Self::fields(inputs, member, index),
                        )
                    })
                    .collect::<Vec<_>>()
            })
        } else if node
            .binding
            .as_ref()
            .is_some_and(|binding| binding.source == GroupDataSource::WorldBosses)
        {
            Some(WorldBossRows::collect(node, inputs))
        } else {
            super::combat_rows::CombatRows::collect(node, inputs)
        };

        let known = rows.is_some();

        let mut rows = rows.unwrap_or_default();

        let limit = node
            .binding
            .as_ref()
            .map_or(128, |binding| binding.limit as usize);

        let group_limit = if node.group.limit > 0.0 {
            node.group.limit as usize
        } else {
            128
        };

        rows.truncate(limit.min(group_limit));

        for child in &node.children {
            let mut base = PreviewVisual::from_node(child);

            base.simulating = true;
            base.active = known.then_some(false);
            base.visible = base.active;

            let mut states = Vec::new();

            for (index, row_data) in rows.iter().enumerate() {
                let key = row_data.key().to_owned();

                let mut copy = child.clone();

                let mut mapping = HashMap::new();

                Self::bind(&mut copy, row_data.entity_id(), &key, &mut mapping);

                let mut fields = row_data.fields().clone();

                fields.insert("row.rank".into(), json!(index + 1));

                let row_inputs = inputs.with_fields(fields);

                let mut row = LiveFrame::new(game.header().status(), None);

                evaluator.node(&copy, &row_inputs, &mut row)?;
                states.extend(row.states().values().copied());

                if let Some(mut visual) = row.visuals().get(&copy.id).cloned() {
                    visual.child_visuals = mapping
                        .iter()
                        .filter_map(|(original, bound)| {
                            (bound != &copy.id)
                                .then(|| {
                                    row.visuals()
                                        .get(bound)
                                        .cloned()
                                        .map(|visual| (original.clone(), visual))
                                })
                                .flatten()
                        })
                        .collect();

                    let instances = std::mem::take(&mut visual.instances);

                    if instances.is_empty() {
                        super::super::row_layout::RowLayout::apply(
                            node,
                            rows.len(),
                            index,
                            &mut visual,
                        );
                        base.instances.push(PreviewInstance {
                            key,
                            visual: Box::new(visual),
                        });
                    } else {
                        for instance in instances {
                            let mut item = *instance.visual;

                            item.child_visuals = visual.child_visuals.clone();
                            super::super::row_layout::RowLayout::apply(
                                node,
                                rows.len(),
                                index,
                                &mut item,
                            );
                            base.instances.push(PreviewInstance {
                                key: format!("{key}:{}", instance.key),
                                visual: Box::new(item),
                            });
                        }
                    }
                }
            }

            let state = if states.contains(&TriggerState::Unsupported) {
                TriggerState::Unsupported
            } else if states.contains(&TriggerState::Active) {
                TriggerState::Active
            } else if !known || states.contains(&TriggerState::Unknown) {
                TriggerState::Unknown
            } else {
                TriggerState::Inactive
            };

            frame.insert(
                child.id.clone(),
                state,
                HashMap::from([(child.id.clone(), base)]),
            );
        }

        Ok(())
    }

    fn bind(
        node: &mut WidgetNode,
        entity_id: Option<EntityId>,
        key: &str,
        mapping: &mut HashMap<String, String>,
    ) {
        let original = node.id.clone();

        node.id = format!("{}@party:{key}", node.id);
        mapping.insert(original, node.id.clone());
        node.entity_id = entity_id.map(|id| id.get().to_string());

        for child in &mut node.children {
            Self::bind(child, entity_id, key, mapping);
        }
    }

    fn fields(
        inputs: &LiveInputs<'_>,
        member: &PartyMember,
        index: usize,
    ) -> HashMap<String, Value> {
        let mut fields = inputs.fields().clone();

        fields.retain(|key, _| !key.starts_with("entity.") && !key.starts_with("row."));

        if let Some(unit) = member
            .entity_id()
            .and_then(|id| inputs.game().entities().get(&id))
        {
            LiveFields::unit(&mut fields, "entity", unit, inputs.game().header().at_us());

            if let Some(statistics) =
                super::combat_projection::CombatProjection::actor(inputs.game(), unit.id())
            {
                super::combat_projection::CombatProjection::fields(
                    &mut fields,
                    "entity",
                    &statistics,
                    super::combat_projection::CombatProjection::duration(inputs.game()),
                );
            }
        }

        fields.insert("row.rank".into(), json!(index + 1));
        fields.insert("row.slot".into(), json!(member.slot()));
        fields.insert("row.id".into(), json!(member.database_id().to_string()));
        fields.insert("row.linked".into(), json!(member.entity_id().is_some()));

        if let Some(name) = member.identity().name() {
            fields.insert("row.name".into(), json!(name));
            fields.insert("entity.name".into(), json!(name));
        }

        if let Some(class) = member.identity().class_name() {
            fields.insert("row.class".into(), json!(class));
            fields.insert("entity.class".into(), json!(class));
        }

        if let Some(level) = member.identity().level() {
            fields.insert("entity.level".into(), json!(level));
        }

        if let Some(server) = member.identity().server_id() {
            fields.insert("entity.serverId".into(), json!(server));
        }

        fields.insert("entity.gearScore".into(), json!(member.gear_score()));
        fields.insert("entity.combatPower".into(), json!(member.combat_power()));

        if let Some(id) = member.entity_id() {
            fields.insert("row.entityId".into(), json!(id.get().to_string()));
        }

        fields
    }
}
