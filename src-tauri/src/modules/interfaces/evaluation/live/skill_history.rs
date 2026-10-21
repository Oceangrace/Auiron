use super::live_frame::LiveFrame;
use super::live_inputs::LiveInputs;
use super::trigger_state::TriggerState;
use crate::core::game::EntityId;
use crate::core::game::EventRecord;
use crate::core::game::GameEvent;
use crate::core::game::GameSnapshot;
use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::PreviewInstance;
use crate::modules::interfaces::evaluation::PreviewVisual;
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
pub(super) struct SkillHistory {
    observation_id: String,
    player_id: Option<EntityId>,
    cursor: u64,
    at_us: u64,
    entries: VecDeque<EventRecord>,
}

impl SkillHistory {
    // Match the maximum binding limit, independently of combat journal churn.
    const LIMIT: usize = 128;
    const RETENTION_US: u64 = 3_600_000_000;

    pub(super) fn update(&mut self, game: &GameSnapshot) {
        if self.observation_id != game.header().observation_id()
            || self.player_id != game.player_id()
            || game.header().at_us() < self.at_us
        {
            self.entries.clear();
            self.cursor = 0;
            self.observation_id = game.header().observation_id().to_owned();
            self.player_id = game.player_id();
        }

        self.at_us = game.header().at_us();

        if !game.player_in_world() {
            self.entries.clear();
            self.cursor = game
                .events()
                .last()
                .map_or(self.cursor, EventRecord::sequence);
            return;
        }

        for record in game.events() {
            if record.sequence() <= self.cursor {
                continue;
            }

            self.cursor = record.sequence();

            // Server acknowledgements belong to the local player, unlike hit/proc events.
            if matches!(record.event(), GameEvent::SkillConfirmed { .. }) {
                self.entries.push_back(record.clone());
            }
        }

        self.entries
            .retain(|record| self.at_us.saturating_sub(record.at_us()) < Self::RETENTION_US);

        while self.entries.len() > Self::LIMIT {
            self.entries.pop_front();
        }
    }

    pub(super) fn populate(
        &self,
        node: &WidgetNode,
        inputs: &LiveInputs<'_>,
        frame: &mut LiveFrame,
    ) {
        let limit = node
            .binding
            .as_ref()
            .map_or(Self::LIMIT, |binding| binding.limit as usize);
        let limit = if node.group.limit > 0.0 {
            limit.min(node.group.limit as usize)
        } else {
            limit
        };

        for child in &node.children {
            let mut base = PreviewVisual::from_node(child);

            base.visible = Some(false);
            base.active = Some(false);

            if child.enabled {
                base.instances = self
                    .entries
                    .iter()
                    .rev()
                    .filter_map(|record| Self::instance(node, child, inputs, record))
                    .take(limit)
                    .collect();
            }

            frame.insert(
                child.id.clone(),
                TriggerState::Active,
                HashMap::from([(child.id.clone(), base)]),
            );
        }
    }

    fn instance(
        node: &WidgetNode,
        child: &WidgetNode,
        inputs: &LiveInputs<'_>,
        record: &EventRecord,
    ) -> Option<PreviewInstance> {
        let GameEvent::SkillConfirmed { skill_id } = record.event() else {
            return None;
        };

        let elapsed = inputs
            .game()
            .header()
            .at_us()
            .saturating_sub(record.at_us()) as f64
            / 1_000_000.0;
        let remaining = node.group.span_seconds - elapsed;

        if remaining <= 0.0 || !inputs.game().player_in_world() {
            return None;
        }

        let mut visual = PreviewVisual::from_node(child);

        visual.visible = Some(true);
        visual.active = Some(true);
        visual.simulating = true;
        visual.remaining = Some(remaining);
        visual.duration = Some(node.group.span_seconds);
        visual.appearance.icon_path = inputs
            .resolver()
            .icon(None, Some(skill_id.get()))
            .map(str::to_owned);
        visual.appearance.custom_image = None;
        visual.appearance.opacity *= remaining.min(1.0);

        Some(PreviewInstance {
            key: format!(
                "{}:{}",
                inputs.game().header().observation_id(),
                record.sequence()
            ),
            visual: Box::new(visual),
        })
    }
}
