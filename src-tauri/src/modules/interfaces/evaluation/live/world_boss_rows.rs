use super::live_inputs::LiveInputs;
use super::live_row::LiveRow;
use super::world_boss_status::WorldBossStatus;
use crate::integrations::aion2::catalog::FieldBossCatalog;
use crate::modules::interfaces::document::WidgetNode;
use serde_json::json;
use std::collections::HashMap;

pub(super) struct WorldBossRows;

impl WorldBossRows {
    pub(super) fn collect(node: &WidgetNode, inputs: &LiveInputs<'_>) -> Vec<LiveRow> {
        let Some(binding) = &node.binding else {
            return Vec::new();
        };

        let mut rows = binding
            .boss_ids
            .iter()
            .filter_map(|id| Self::row(*id, inputs))
            .collect::<Vec<_>>();

        rows.sort_by_key(|(rank, time, _)| (*rank, *time));

        rows.into_iter().map(|(_, _, row)| row).collect()
    }

    fn row(id: u32, inputs: &LiveInputs<'_>) -> Option<(u8, u64, LiveRow)> {
        let definition = FieldBossCatalog::find(id)?;

        let game = inputs.game();

        let now = game.header().at_us();

        let list = game
            .world()
            .boss_lists()
            .iter()
            .find(|list| list.map_id().get() == definition.map_id);

        let observed =
            list.and_then(|list| list.bosses().iter().find(|boss| boss.world_map_id() == id));

        let age = list.map(|list| now.saturating_sub(list.observed_at_us()));

        let remaining = observed
            .filter(|boss| !boss.alive())
            .and_then(|boss| boss.time_ms())
            .filter(|time| *time > now / 1000)
            .map(|time| time - now / 1000);

        let (rank, status) = match observed {
            Some(boss) if boss.alive_at(now / 1000) => (0, WorldBossStatus::ObservedAlive),
            Some(_) if remaining.is_some() => (1, WorldBossStatus::Countdown),
            Some(_) => (2, WorldBossStatus::Refresh),
            None => (3, WorldBossStatus::Waiting),
        };

        let name = inputs
            .resolver()
            .npc_name(definition.npc_id, inputs.text().locale())
            .unwrap_or_else(|| format!("#{id}"));

        let fields = HashMap::from([
            ("row.id".into(), json!(id.to_string())),
            ("row.name".into(), json!(name)),
            ("row.status".into(), json!(status)),
            ("row.remaining".into(), json!(remaining)),
            ("row.timer".into(), json!(remaining.map(Self::timer))),
            ("row.observedAge".into(), json!(age.map(|age| age / 1000))),
            ("row.mapId".into(), json!(definition.map_id)),
        ]);

        Some((
            rank,
            remaining.unwrap_or(0),
            LiveRow::new(id.to_string(), None, fields),
        ))
    }

    fn timer(remaining_ms: u64) -> String {
        let seconds = remaining_ms.div_ceil(1000);

        if seconds >= 3600 {
            return format!(
                "{}:{:02}:{:02}",
                seconds / 3600,
                seconds / 60 % 60,
                seconds % 60
            );
        }

        format!("{}:{:02}", seconds / 60, seconds % 60)
    }
}
