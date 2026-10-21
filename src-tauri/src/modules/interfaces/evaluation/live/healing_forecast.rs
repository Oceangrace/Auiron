use super::healing_series::HealingSeries;
use crate::core::game::EffectInstanceId;
use crate::core::game::EffectSnapshot;
use crate::core::game::EffectTiming;
use crate::core::game::EntityId;
use crate::core::game::EventRecord;
use crate::core::game::GameEvent;
use crate::core::game::GameSnapshot;
use crate::core::game::SkillId;
use crate::core::game::SourceStatus;
use crate::core::game::UnitSnapshot;
use crate::modules::interfaces::appearance::HealingPredictionTarget;
use crate::modules::interfaces::document::WidgetKind;
use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::HealingPrediction;
use crate::modules::interfaces::evaluation::PreviewVisual;
use std::collections::{HashMap, HashSet, VecDeque};

/// Effect key: target, effect instance, deadline and the SkillEffect row of the healing effect.
type SampleKey = (EntityId, EffectInstanceId, u64, Option<u8>);

type AnnouncementKey = (EntityId, EntityId, EffectInstanceId, SkillId, u8, u64);

#[derive(Default)]
pub(super) struct HealingForecast {
    observation_id: String,
    last: Option<(u64, u64)>,
    samples: HashMap<SampleKey, HealingSeries>,
    pending: VecDeque<EventRecord>,
    announced: HashMap<AnnouncementKey, u32>,
    // Effects whose announcement matched the first tick (pool = remaining + tick). Kept for
    // the whole run: from then on the forecast starts at the announcement, not the first tick.
    confirmed: HashSet<(SkillId, u8)>,
}

impl HealingForecast {
    const MAX_PENDING: usize = 128;
    const MAX_TRACKED: usize = 4096;
    /// How long a pool announcement may wait for its effect to appear.
    const ANNOUNCEMENT_WAIT_US: u64 = 2_000_000;
    /// Server-to-local clock conversion error tolerated when matching an effect.
    const CLOCK_JITTER_US: u64 = 250_000;

    pub(super) fn update(&mut self, game: &GameSnapshot) {
        if self.observation_id != game.header().observation_id() {
            self.samples.clear();
            self.announced.clear();
            self.pending.clear();
            self.last = None;
            self.observation_id = game.header().observation_id().to_owned();
        }

        if !matches!(
            game.header().status(),
            SourceStatus::Live | SourceStatus::Replay
        ) {
            self.samples.clear();
            self.announced.clear();
            self.pending.clear();

            return;
        }

        self.samples.retain(|(entity, instance, deadline, _), _| {
            game.entities().get(entity).is_some_and(|unit| {
                unit.present()
                    && unit.effects().items().iter().any(|effect| {
                        effect.effect().instance_id() == *instance
                            && Self::deadline(effect) == Some(*deadline)
                            && effect
                                .expires_at_us()
                                .is_some_and(|end| end > game.header().at_us())
                    })
            })
        });

        self.retry_announcements(game);

        for record in game.events() {
            let position = (record.at_us(), record.sequence());

            if self.last.is_some_and(|previous| position <= previous) {
                continue;
            }

            self.last = Some(position);

            if matches!(record.event(), GameEvent::HealingPool { .. }) {
                if !self.announce(game, record) {
                    if self.pending.len() >= Self::MAX_PENDING {
                        self.pending.pop_front();
                    }

                    self.pending.push_back(record.clone());
                }

                continue;
            }

            let GameEvent::Healing {
                entity_id,
                source_id,
                skill_id,
                value,
                hit,
            } = record.event()
            else {
                continue;
            };

            if !hit.as_ref().is_some_and(|hit| hit.periodic()) {
                continue;
            }

            let pool = hit.as_ref().and_then(|hit| hit.pool_remaining());

            let row = hit.as_ref().and_then(|hit| hit.pool_effect_row());

            if let Some((entity, instance, deadline)) =
                Self::key(game, *entity_id, *source_id, *skill_id, record.at_us())
            {
                if let (Some(pool), Some(row)) = (pool, row)
                    && self
                        .announced
                        .remove(&(entity, *source_id, instance, *skill_id, row, deadline))
                        .is_some_and(|announced| Some(announced) == pool.checked_add(*value))
                    && self.confirmed.len() < Self::MAX_TRACKED
                {
                    self.confirmed.insert((*skill_id, row));
                }

                if pool.is_some() {
                    self.samples.remove(&(entity, instance, deadline, None));
                }

                self.samples
                    .entry((entity, instance, deadline, row))
                    .or_default()
                    .observe(record.at_us(), *value, pool);
            }
        }
    }

    fn retry_announcements(&mut self, game: &GameSnapshot) {
        let pending = std::mem::take(&mut self.pending);

        for record in pending {
            if game.header().at_us().saturating_sub(record.at_us()) <= Self::ANNOUNCEMENT_WAIT_US
                && !self.announce(game, &record)
            {
                self.pending.push_back(record);
            }
        }
    }

    fn announce(&mut self, game: &GameSnapshot, record: &EventRecord) -> bool {
        let GameEvent::HealingPool {
            entity_id,
            source_id,
            instance_id,
            skill_id,
            effect_row,
            remaining,
        } = record.event()
        else {
            return true;
        };

        // Allow clock conversion jitter and a pool arriving just before its effect state.
        let Some(effect) = game
            .entities()
            .get(entity_id)
            .filter(|unit| unit.present())
            .and_then(|unit| {
                unit.effects().items().iter().find(|effect| {
                    effect.effect().instance_id() == *instance_id
                        && effect.effect().source_id() == Some(*source_id)
                        && effect.effect().skill_id() == Some(*skill_id)
                        && effect.applied_at_us()
                            <= record.at_us().saturating_add(Self::CLOCK_JITTER_US)
                        && effect
                            .expires_at_us()
                            .is_some_and(|end| end > record.at_us())
                })
            })
        else {
            return false;
        };

        let Some(deadline) = Self::deadline(effect) else {
            return false;
        };

        if self.announced.len() >= Self::MAX_TRACKED {
            self.announced.clear();
        }

        self.announced.insert(
            (
                *entity_id,
                *source_id,
                *instance_id,
                *skill_id,
                *effect_row,
                deadline,
            ),
            *remaining,
        );

        if effect.is_healing_row(*effect_row) || self.confirmed.contains(&(*skill_id, *effect_row))
        {
            self.samples
                .entry((*entity_id, *instance_id, deadline, Some(*effect_row)))
                .or_default()
                .announce(*remaining);
        }

        true
    }

    pub(super) fn apply(&self, node: &WidgetNode, game: &GameSnapshot, visual: &mut PreviewVisual) {
        if node.kind != WidgetKind::Bar || !visual.known_progress {
            return;
        }

        let id = match node.style.healing_prediction {
            Some(HealingPredictionTarget::Player) => game.player_id(),
            Some(HealingPredictionTarget::Target) => game.target().entity_id(),
            Some(HealingPredictionTarget::Entity) => node
                .entity_id
                .as_deref()
                .and_then(|id| id.parse().ok())
                .and_then(|id| EntityId::new(id).ok()),
            None => None,
        };

        let Some(unit) = id
            .and_then(|id| game.entities().get(&id))
            .filter(|unit| unit.present())
        else {
            return;
        };

        let amount = unit
            .effects()
            .items()
            .iter()
            .filter(|effect| Self::unambiguous(unit, effect))
            .flat_map(|effect| {
                let instance = effect.effect().instance_id();

                let deadline = Self::deadline(effect);

                self.samples
                    .iter()
                    .filter(move |((entity, sample, at, _), _)| {
                        *entity == unit.id() && *sample == instance && Some(*at) == deadline
                    })
                    .filter_map(|(_, series)| {
                        series.remaining(game.header().at_us(), effect.expires_at_us()?)
                    })
            })
            .sum();

        visual.healing_prediction = HealingPrediction::new(unit.health(), amount);
    }

    fn unambiguous(unit: &UnitSnapshot, candidate: &EffectSnapshot) -> bool {
        unit.effects()
            .items()
            .iter()
            .filter(|effect| {
                effect.effect().source_id() == candidate.effect().source_id()
                    && effect.effect().skill_id() == candidate.effect().skill_id()
            })
            .count()
            == 1
    }

    fn deadline(effect: &EffectSnapshot) -> Option<u64> {
        match effect.effect().timing() {
            EffectTiming::Timed {
                ends_at_server_ms, ..
            } => *ends_at_server_ms,
            EffectTiming::Untimed => None,
        }
    }

    fn key(
        game: &GameSnapshot,
        entity: EntityId,
        source: EntityId,
        skill: SkillId,
        at_us: u64,
    ) -> Option<(EntityId, EffectInstanceId, u64)> {
        let unit = game.entities().get(&entity).filter(|unit| unit.present())?;

        let mut effects = unit.effects().items().iter().filter(|effect| {
            effect.effect().source_id() == Some(source)
                && effect.effect().skill_id() == Some(skill)
                && effect.applied_at_us() <= at_us
                && effect.expires_at_us().is_some_and(|end| end > at_us)
        });

        let effect = effects.next()?;

        if effects.next().is_some() {
            return None;
        }

        Some((
            entity,
            effect.effect().instance_id(),
            Self::deadline(effect)?,
        ))
    }
}
