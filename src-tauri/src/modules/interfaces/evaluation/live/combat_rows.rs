use super::combat_projection::CombatProjection;
use super::live_fields::LiveFields;
use super::live_inputs::LiveInputs;
use super::live_row::LiveRow;
use crate::core::game::CombatCounter;
use crate::core::game::CombatStatistics;
use crate::core::game::EntityId;
use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::layout::CombatDirection;
use crate::modules::interfaces::layout::CombatScope;
use crate::modules::interfaces::layout::TableDimension;
use serde_json::json;
use std::collections::BTreeMap;

pub(super) struct CombatRows;

impl CombatRows {
    pub(super) fn collect(node: &WidgetNode, inputs: &LiveInputs<'_>) -> Option<Vec<LiveRow>> {
        let game = inputs.game();

        let mut binding = node.binding.clone()?;

        if let Some(view) = binding
            .views
            .iter()
            .find(|view| Some(&view.id) == binding.active_view_id.as_ref())
        {
            binding.metric = view.metric;
            binding.direction = view.direction;
            binding.dimension = view.dimension;
            binding.scope = view.scope;
            binding.actor_id = view.actor_id.clone();
            binding.target_id = view.target_id.clone();
        }

        let focus = match binding.scope {
            CombatScope::Player => Some(game.player_id()?),
            CombatScope::Entity => Some(Self::id(node.entity_id.as_deref())?),
            CombatScope::Party => {
                game.party()?;
                None
            }

            CombatScope::Observed => None,
        };

        let actor = Self::id(binding.actor_id.as_deref());

        let target = Self::id(binding.target_id.as_deref());

        let mut groups = BTreeMap::<(Option<EntityId>, Option<u32>), CombatStatistics>::new();

        for record in game.statistics() {
            let source = if binding.include_summons {
                match game.root_actor(record.source_id()) {
                    Some(id) => id,
                    None => continue,
                }
            } else {
                record.source_id()
            };

            let recipient = record.recipient_id();

            let subject = if binding.direction == CombatDirection::Outgoing {
                source
            } else {
                recipient
            };

            if focus.is_some_and(|id| id != subject)
                || (binding.scope == CombatScope::Party && game.group_member(subject) != Some(true))
                || actor.is_some_and(|id| id != source)
                || target.is_some_and(|id| id != recipient)
            {
                continue;
            }

            let key = match binding.dimension {
                TableDimension::Combatants => (Some(subject), None),
                TableDimension::Skills => (focus, record.skill_id().map(|id| id.get())),
                TableDimension::Targets => (
                    Some(if binding.direction == CombatDirection::Outgoing {
                        recipient
                    } else {
                        source
                    }),
                    None,
                ),
            };

            let statistics = groups.entry(key).or_default();

            if source == record.source_id() {
                statistics.merge(record.statistics());
            } else {
                statistics.merge_owned_actions(record.statistics());
            }

            if binding.direction == CombatDirection::Incoming {
                statistics.increment(
                    CombatCounter::DamageTaken,
                    record.statistics().count(CombatCounter::Damage),
                );
                statistics.increment(
                    CombatCounter::HealingTaken,
                    record.statistics().count(CombatCounter::Healing),
                );
                statistics.increment(
                    CombatCounter::ReceivedHits,
                    record.statistics().count(CombatCounter::Hits),
                );
            }
        }

        let duration = CombatProjection::duration(game);

        let mut rows: Vec<_> = groups
            .into_iter()
            .map(|((entity, skill), statistics)| {
                let mut fields = inputs.fields().clone();

                fields.retain(|key, _| !key.starts_with("entity.") && !key.starts_with("row."));

                let key = if binding.dimension == TableDimension::Skills {
                    format!(
                        "skill:{}",
                        skill.map_or_else(|| "unknown".into(), |id| id.to_string())
                    )
                } else {
                    format!(
                        "entity:{}",
                        entity.map_or_else(|| "unknown".into(), |id| id.get().to_string())
                    )
                };

                let mut name = skill
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| key.clone());

                if let Some(unit) = entity.and_then(|id| game.entities().get(&id)) {
                    LiveFields::unit(&mut fields, "entity", unit, game.header().at_us());

                    if binding.dimension != TableDimension::Skills {
                        name = unit.identity().name().unwrap_or(&name).to_owned();
                    }

                    if let Some(class) = unit.identity().class_name() {
                        fields.insert("row.class".into(), json!(class));
                    }
                }

                fields.insert("row.id".into(), json!(key));
                fields.insert("row.name".into(), json!(name));

                if let Some(entity) = entity {
                    fields.insert("row.entityId".into(), json!(entity.get().to_string()));
                }

                if let Some(skill) = skill {
                    fields.insert("row.skillId".into(), json!(skill));
                }

                CombatProjection::fields(&mut fields, "row", &statistics, duration);

                if let Some(value) = CombatProjection::value(&statistics, binding.metric, duration)
                {
                    fields.insert("row.value".into(), json!(value));
                }

                LiveRow::new(key, entity, fields)
            })
            .collect();

        rows.sort_by(|left, right| {
            let left = left
                .fields()
                .get("row.value")
                .and_then(|value| value.as_f64());

            let right = right
                .fields()
                .get("row.value")
                .and_then(|value| value.as_f64());

            match (left, right) {
                (Some(left), Some(right)) => right.total_cmp(&left),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        });

        let values: Option<Vec<f64>> = rows
            .iter()
            .map(|row| {
                row.fields()
                    .get("row.value")
                    .and_then(|value| value.as_f64())
            })
            .collect();

        let total = values.as_ref().map(|values| values.iter().sum());

        let leader = rows
            .first()
            .and_then(|row| row.fields().get("row.value"))
            .and_then(|value| value.as_f64());

        for row in &mut rows {
            row.percentages(total, leader);
        }

        Some(rows)
    }

    fn id(value: Option<&str>) -> Option<EntityId> {
        value?.parse().ok().and_then(|id| EntityId::new(id).ok())
    }
}
