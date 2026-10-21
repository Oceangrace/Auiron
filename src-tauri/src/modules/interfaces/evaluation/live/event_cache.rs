use super::binding_key::BindingKey;
use super::cached_event::CachedEvent;
use super::trigger_match::TriggerMatch;
use super::trigger_result::TriggerResult;
use super::trigger_state::TriggerState;
use crate::core::game::GameEvent;
use crate::core::game::GameSnapshot;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub(super) struct EventCache {
    observation_id: String,
    entries: HashMap<BindingKey, CachedEvent>,
    used: HashSet<BindingKey>,
}

impl EventCache {
    pub(super) fn begin(&mut self, game: &GameSnapshot) {
        if self.observation_id != game.header().observation_id() {
            self.entries.clear();
            self.observation_id = game.header().observation_id().to_owned();
        }

        self.used.clear();
    }

    fn current_cast(event: &GameEvent, game: &GameSnapshot) -> bool {
        let Some(preparation) = event.cast_preparation() else {
            return true;
        };

        event
            .source()
            .and_then(|id| game.entities().get(&id))
            .and_then(|unit| unit.cast())
            .is_some_and(|cast| {
                cast.started_at_us() == preparation.started_at_us()
                    && cast.skill_id() == preparation.skill_id()
            })
    }

    pub(super) fn finish(&mut self) {
        self.entries.retain(|key, _| self.used.contains(key));
    }

    pub(super) fn evaluate(
        &mut self,
        key: BindingKey,
        signature: String,
        game: &GameSnapshot,
        window: (f64, f64),
        matches: impl Fn(&GameEvent) -> Option<bool>,
    ) -> TriggerResult {
        self.used.insert(key.clone());

        if self.entries.get(&key).is_some_and(|cached| {
            cached.signature() != signature || matches(cached.record().event()) != Some(true)
        }) {
            self.entries.remove(&key);
        }

        let candidate = game
            .events()
            .iter()
            .rev()
            .find(|record| matches(record.event()) == Some(true));

        if let Some(record) = candidate
            && self.entries.get(&key).is_none_or(|cached| {
                (record.at_us(), record.sequence())
                    > (cached.record().at_us(), cached.record().sequence())
            })
        {
            self.entries
                .insert(key.clone(), CachedEvent::new(signature, record.clone()));
        }

        let uncertain = game
            .events()
            .iter()
            .any(|record| matches(record.event()).is_none());

        let Some(cached) = self.entries.get(&key) else {
            return TriggerResult::new(
                if uncertain {
                    TriggerState::Unknown
                } else {
                    TriggerState::Inactive
                },
                Vec::new(),
            );
        };

        let elapsed = game
            .header()
            .at_us()
            .saturating_sub(cached.record().at_us()) as f64
            / 1000.0;

        let (duration_ms, delay_ms) = window;

        let (duration_ms, delay_ms) = cached
            .record()
            .event()
            .cast_preparation()
            .map_or((duration_ms, delay_ms), |cast| {
                (f64::from(cast.duration_ms()), 0.0)
            });

        let active = elapsed < duration_ms && Self::current_cast(cached.record().event(), game);

        TriggerResult::new(
            TriggerState::from_active(Some(active), false),
            vec![TriggerMatch::Event {
                record: cached.record().clone(),
                duration_ms,
                delay_ms,
            }],
        )
    }
}
