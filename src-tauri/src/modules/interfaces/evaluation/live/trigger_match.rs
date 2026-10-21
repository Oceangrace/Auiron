use crate::core::game::CooldownBasis;
use crate::core::game::CooldownSnapshot;
use crate::core::game::EffectSnapshot;
use crate::core::game::EffectTiming;
use crate::core::game::EntityId;
use crate::core::game::EventRecord;
use crate::core::game::GameEvent;
use crate::core::game::GameSnapshot;
use crate::core::game::HitOutcome;
use serde_json::{Value, json};
use std::collections::HashMap;

#[derive(Clone)]
pub(super) enum TriggerMatch {
    Boss {
        fields: HashMap<String, Value>,
    },
    Cooldown {
        cooldown: CooldownSnapshot,
    },
    Effect {
        entity_id: EntityId,
        instances: Vec<EffectSnapshot>,
    },
    Event {
        record: EventRecord,
        duration_ms: f64,
        delay_ms: f64,
    },
}

impl TriggerMatch {
    pub(super) fn icon_visible(&self) -> bool {
        match self {
            Self::Effect { instances, .. } => instances.first().is_some_and(|effect| {
                effect
                    .display()
                    .is_none_or(|display| display.icon_visible())
            }),
            _ => true,
        }
    }

    pub(super) fn fields(&self, game: &GameSnapshot) -> HashMap<String, Value> {
        let now_us = game.header().at_us();

        let mut fields = HashMap::new();

        match self {
            Self::Boss { fields: values } => fields.extend(values.clone()),
            Self::Cooldown { cooldown } => {
                fields.insert("remaining".into(), json!(cooldown.remaining_ms()));
                fields.insert("groupId".into(), json!(cooldown.group_id().get()));
                fields.insert(
                    "estimated".into(),
                    json!(cooldown.basis() == CooldownBasis::BaseDuration),
                );

                if let Some(duration) = cooldown.duration_ms() {
                    fields.insert("duration".into(), json!(duration));
                }

                if let Some(skill) = cooldown.skill_id() {
                    fields.insert("skillId".into(), json!(skill.get()));
                }

                if let Some(ready) = cooldown.ready() {
                    fields.insert("ready".into(), json!(ready));
                }
            }

            Self::Effect {
                entity_id,
                instances,
            } => {
                let selected = instances
                    .iter()
                    .min_by_key(|effect| effect.expires_at_us().unwrap_or(u64::MAX));

                fields.insert("entityId".into(), json!(entity_id.get().to_string()));
                fields.insert("stacks".into(), json!(instances.len()));

                if let Some(selected) = selected {
                    if let Some(icon) = selected.display().and_then(|display| display.icon()) {
                        fields.insert("iconPath".into(), json!(icon));
                    }

                    fields.insert("level".into(), json!(selected.effect().level()));
                    fields.insert("unit".into(), json!(entity_id.get().to_string()));

                    if let Some(kind) = selected.disposition() {
                        fields.insert("kind".into(), json!(kind));
                    }

                    if let Some(source) = selected.effect().source_id() {
                        fields.insert("source".into(), json!(source.get().to_string()));
                    }

                    if let Some(skill) = selected.effect().skill_id() {
                        fields.insert("skillId".into(), json!(skill.get()));
                    }

                    if let Some(end) = selected.expires_at_us() {
                        fields.insert("expires".into(), json!(end / 1000));
                    }

                    fields.insert(
                        "effectId".into(),
                        json!(selected.effect().effect_id().get()),
                    );
                    fields.insert(
                        "instanceId".into(),
                        json!(selected.effect().instance_id().get()),
                    );
                    fields.insert(
                        "elapsed".into(),
                        json!(now_us.saturating_sub(selected.applied_at_us()) as f64 / 1000.0),
                    );

                    if let Some(end) = selected.expires_at_us() {
                        fields.insert(
                            "remaining".into(),
                            json!(end.saturating_sub(now_us) as f64 / 1000.0),
                        );
                    }

                    if let EffectTiming::Timed { duration_ms, .. } = selected.effect().timing() {
                        fields.insert("duration".into(), json!(duration_ms));
                    }
                }
            }

            Self::Event {
                record,
                duration_ms,
                delay_ms,
            } => {
                let elapsed = now_us.saturating_sub(record.at_us()) as f64 / 1000.0;

                let remaining = (duration_ms - elapsed).max(0.0);

                fields.insert("id".into(), json!(record.sequence().to_string()));
                fields.insert("type".into(), json!(record.event().event_type()));
                fields.insert(
                    "duration".into(),
                    json!(if *delay_ms > 0.0 {
                        *delay_ms
                    } else {
                        *duration_ms
                    }),
                );
                fields.insert("remaining".into(), json!(remaining));

                if *delay_ms > 0.0 {
                    fields.insert("countdown".into(), json!((delay_ms - elapsed).max(0.0)));
                }

                fields.insert("elapsed".into(), json!(elapsed));

                if let GameEvent::EffectRemoved {
                    reason, removed_by, ..
                } = record.event()
                {
                    fields.insert("removalReason".into(), json!(reason));

                    if let Some(id) = removed_by {
                        fields.insert("removedBy".into(), json!(id.get().to_string()));
                    }
                }

                if let GameEvent::ComboAvailable {
                    previous_skill_id, ..
                } = record.event()
                {
                    fields.insert("previousSkillId".into(), json!(previous_skill_id.get()));
                }

                if let Some(value) = record.event().value() {
                    fields.insert("value".into(), json!(value));
                }

                if let Some(outcome) = record.event().outcome() {
                    fields.insert("outcome".into(), json!(outcome));
                }

                if let Some(hit) = record.event().hit() {
                    if let Some(outcome) = hit.outcome() {
                        fields.insert("critical".into(), json!(outcome == HitOutcome::Critical));
                        fields.insert("hitType".into(), json!(outcome));
                    }

                    fields.insert("periodic".into(), json!(hit.periodic()));
                    fields.insert(
                        "modifiers".into(),
                        json!(
                            hit.modifiers()
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    );
                    fields.insert("additionalHits".into(), json!(hit.additional_hits()));
                    fields.insert("additionalDamage".into(), json!(hit.additional_damage()));

                    if !hit.protections().is_empty() {
                        fields.insert("protections".into(), json!(hit.protections().len()));
                    }

                    if let Some(life_steal) = hit.life_steal() {
                        fields.insert("lifeSteal".into(), json!(life_steal));
                    }

                    if let Some(scalar) = hit.power_scalar() {
                        fields.insert("powerScalar".into(), json!(scalar));
                    }

                    if let Some(restored) = hit.restored_hp() {
                        fields.insert("restoredHp".into(), json!(restored));
                    }
                }

                if let Some(source) = record.event().source() {
                    fields.insert("sourceId".into(), json!(source.get().to_string()));

                    if let Some(name) = game
                        .entities()
                        .get(&source)
                        .and_then(|unit| unit.identity().name())
                    {
                        fields.insert("sourceName".into(), json!(name));
                    }
                }

                if let Some(recipient) = record.event().recipient() {
                    fields.insert("entityId".into(), json!(recipient.get().to_string()));

                    if let Some(name) = game
                        .entities()
                        .get(&recipient)
                        .and_then(|unit| unit.identity().name())
                    {
                        fields.insert("entityName".into(), json!(name));
                    }
                }

                if let Some(effect) = record.event().effect_id() {
                    fields.insert("effectId".into(), json!(effect.get()));
                }

                if let Some(instance) = record.event().instance_id() {
                    fields.insert("instanceId".into(), json!(instance.get()));
                }

                if let Some(skill) = record.event().skill_id() {
                    fields.insert("skillId".into(), json!(skill.get()));
                }
            }
        }

        fields
    }
}
