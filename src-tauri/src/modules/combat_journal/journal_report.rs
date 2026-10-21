use super::journal_action::JournalAction;
use super::journal_effect::JournalEffect;
use super::journal_period::JournalPeriod;
use super::journal_resource::JournalResource;
use super::journal_unit::JournalUnit;
use crate::core::game::CombatRecord;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct JournalReport {
    id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    period: Option<JournalPeriod>,
    started_at_us: u64,
    ended_at_us: Option<u64>,
    elapsed_ms: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    span_ms: Option<f64>,
    map_id: Option<u32>,
    #[serde(default)]
    dungeon_id: Option<u32>,
    #[serde(default)]
    encounter_id: Option<u64>,
    npc_id: Option<u32>,
    player_id: Option<u64>,
    units: BTreeMap<u64, JournalUnit>,
    records: Vec<CombatRecord>,
    #[serde(default)]
    actions: Vec<JournalAction>,
    #[serde(default)]
    trace_partial: bool,
    #[serde(default)]
    effects: Vec<JournalEffect>,
    #[serde(default)]
    resources: Vec<JournalResource>,
    #[serde(default)]
    resources_partial: bool,
}

impl JournalReport {
    pub(super) fn has_trace(&self) -> bool {
        !self.actions.is_empty() || !self.effects.is_empty()
    }

    pub(super) fn drop_trace(&mut self) {
        self.actions = Vec::new();
        self.effects = Vec::new();
        self.trace_partial = true;
    }

    pub(super) fn validate(&self) -> Result<(), String> {
        if self.id.is_empty()
            || self.id.len() > 100
            || !self.elapsed_ms.is_finite()
            || !(0.0..=604_800_000.0).contains(&self.elapsed_ms)
            || self
                .span_ms
                .is_some_and(|span| !span.is_finite() || !(0.0..=604_800_000.0).contains(&span))
            || self.ended_at_us.is_none_or(|end| end < self.started_at_us)
            || self.units.len() > 10_000
            || self.records.len() > 100_000
            || self.actions.len() > 12_000
            || self.effects.len() > 12_000
            || self.resources.len() > 12_000
        {
            return Err("Invalid combat report".into());
        }

        Ok(())
    }
}
