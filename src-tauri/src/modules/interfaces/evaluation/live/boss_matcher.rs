use super::live_inputs::LiveInputs;
use super::trigger_match::TriggerMatch;
use super::trigger_result::TriggerResult;
use super::trigger_state::TriggerState;
use crate::integrations::aion2::catalog::FieldBossCatalog;
use crate::modules::interfaces::rules::BossEvent;
use crate::modules::interfaces::rules::BossTrigger;
use serde_json::json;
use std::collections::HashMap;

pub(super) struct BossMatcher;

impl BossMatcher {
    pub(super) fn evaluate(trigger: &BossTrigger, inputs: &LiveInputs<'_>) -> TriggerResult {
        let Some(definition) = FieldBossCatalog::find(trigger.boss_id) else {
            return TriggerResult::new(TriggerState::Unsupported, Vec::new());
        };

        let game = inputs.game();

        let now = game.header().at_us() as f64 / 1000.0;

        let boss = game
            .world()
            .boss_lists()
            .iter()
            .find(|list| list.map_id().get() == definition.map_id)
            .and_then(|list| {
                list.bosses()
                    .iter()
                    .find(|boss| boss.world_map_id() == trigger.boss_id)
            });

        let deadline = boss
            .and_then(|boss| boss.time_ms())
            .filter(|time| *time > 0);

        let alive = boss.is_some_and(|boss| boss.alive_at(game.header().at_us() / 1000));

        let known = alive || deadline.is_some();

        let starts = deadline.map(|time| {
            time as f64
                - if trigger.event == BossEvent::BeforeSpawn {
                    trigger.before_seconds * 1000.0
                } else {
                    0.0
                }
        });

        let elapsed = starts.map(|start| now - start);

        let active = match trigger.event {
            BossEvent::Unknown => Some(!known),
            BossEvent::Alive => known.then_some(alive),
            BossEvent::BeforeSpawn | BossEvent::Spawned => deadline.map(|_| {
                elapsed.is_some_and(|elapsed| elapsed >= 0.0 && elapsed < trigger.duration_ms)
                    && (trigger.event == BossEvent::Spawned || !alive)
            }),
        };

        let mut fields = HashMap::from([
            ("bossId".into(), json!(trigger.boss_id)),
            ("mapId".into(), json!(definition.map_id)),
            (
                "name".into(),
                json!(
                    inputs
                        .resolver()
                        .npc_name(definition.npc_id, inputs.text().locale())
                ),
            ),
            ("known".into(), json!(known)),
            ("alive".into(), json!(alive)),
            ("respawnsAt".into(), json!(deadline)),
            (
                "countdown".into(),
                json!(deadline.map(|time| (time as f64 - now).max(0.0))),
            ),
            ("duration".into(), json!(trigger.duration_ms)),
            (
                "remaining".into(),
                json!(elapsed.map(|time| (trigger.duration_ms - time.max(0.0)).max(0.0))),
            ),
            ("elapsed".into(), json!(elapsed.map(|time| time.max(0.0)))),
        ]);

        if let Some(deadline) = deadline {
            fields.insert(
                "id".into(),
                json!(format!("{}:{deadline}", trigger.boss_id)),
            );
        }

        TriggerResult::new(
            TriggerState::from_active(active, false),
            vec![TriggerMatch::Boss { fields }],
        )
    }
}
