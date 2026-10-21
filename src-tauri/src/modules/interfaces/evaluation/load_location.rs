use crate::integrations::aion2::GameId;
use crate::modules::interfaces::evaluation::RuleEvaluator;
use crate::modules::interfaces::rules::LoadRules;
use serde_json::Value;
use std::collections::HashMap;

pub(super) struct LoadLocation;

impl LoadLocation {
    pub(super) fn matches(load: &LoadRules, fields: &HashMap<String, Value>) -> Option<bool> {
        let mut places = Vec::new();

        if !load.map_ids.is_empty() {
            places.push(Self::id(&load.map_ids, fields.get("zone.id")));
        }

        if !load.dungeon_ids.is_empty() {
            places.push(Self::id(&load.dungeon_ids, fields.get("group.dungeonId")));
        }

        let place = if places.is_empty() {
            Some(true)
        } else {
            RuleEvaluator::combine(true, &places)
        };

        let npc = if load.npc_ids.is_empty() {
            Some(true)
        } else {
            Self::id(&load.npc_ids, fields.get("encounter.npcId"))
        };

        RuleEvaluator::combine(false, &[place, npc])
    }

    fn id(ids: &[GameId], value: Option<&Value>) -> Option<bool> {
        let actual = value?.as_u64()?;

        Some(ids.iter().any(|id| u64::from(id.get()) == actual))
    }
}
