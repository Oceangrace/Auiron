use super::trigger_match::TriggerMatch;
use super::trigger_result::TriggerResult;
use super::trigger_state::TriggerState;
use crate::core::game::EffectDisposition;
use crate::core::game::EffectId;
use crate::core::game::EffectSnapshot;
use crate::core::game::EntityId;
use crate::core::game::GameSnapshot;
use crate::modules::interfaces::rules::EffectFilter;
use crate::modules::interfaces::rules::EffectOrigin;
use crate::modules::interfaces::rules::EffectPresence;
use crate::modules::interfaces::rules::EffectTrigger;
use crate::modules::interfaces::rules::TriggerUnit;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub(super) struct EffectMatcher;

impl EffectMatcher {
    pub(super) fn supported(trigger: &EffectTrigger, game: &GameSnapshot) -> bool {
        game.capabilities().effects()
            && (matches!(
                trigger.unit,
                TriggerUnit::Player
                    | TriggerUnit::Target
                    | TriggerUnit::Entity
                    | TriggerUnit::Encounter
            ) || game.capabilities().party())
    }

    pub(super) fn evaluate(
        trigger: &EffectTrigger,
        entity: Option<EntityId>,
        game: &GameSnapshot,
    ) -> TriggerResult {
        if !Self::supported(trigger, game) {
            return TriggerResult::new(TriggerState::Unsupported, Vec::new());
        }

        if matches!(
            trigger.unit,
            TriggerUnit::Party | TriggerUnit::Group | TriggerUnit::Member
        ) {
            return Self::group(trigger, game);
        }

        Self::unit(trigger, entity, game)
    }

    fn group(trigger: &EffectTrigger, game: &GameSnapshot) -> TriggerResult {
        if trigger.unit == TriggerUnit::Party && game.player_id().is_none() {
            return TriggerResult::new(TriggerState::Unknown, Vec::new());
        }

        let roster = game.party();

        let mut ids = BTreeSet::new();

        let mut uncertain = roster.is_none_or(|roster| !roster.complete());

        if trigger.unit == TriggerUnit::Member {
            let Some(roster) = roster else {
                return TriggerResult::new(TriggerState::Unknown, Vec::new());
            };

            let mut matches = roster.members().iter().filter(|member| {
                member.identity().name() == Some(trigger.member.as_str())
                    || trigger.member.parse::<u8>().ok() == Some(member.slot())
            });

            let id = matches.next().and_then(|member| member.entity_id());

            if matches.next().is_some() {
                return TriggerResult::new(TriggerState::Unknown, Vec::new());
            }

            return Self::unit(trigger, id, game);
        }

        if trigger.unit == TriggerUnit::Group {
            if let Some(player) = game.player_id() {
                ids.insert(player);
            } else {
                uncertain = true;
            }
        }

        if let Some(roster) = roster {
            for member in roster.members() {
                match member.entity_id() {
                    Some(id)
                        if trigger.unit == TriggerUnit::Group || Some(id) != game.player_id() =>
                    {
                        ids.insert(id);
                    }

                    Some(_) => {}
                    None => uncertain = true,
                }
            }
        }

        let mut active = false;

        let mut matches = Vec::new();

        for id in ids {
            let result = Self::unit(trigger, Some(id), game);

            active |= result.state() == TriggerState::Active;
            uncertain |= result.state() == TriggerState::Unknown;
            matches.extend(result.matches().iter().cloned());
        }

        let state = if active {
            TriggerState::Active
        } else if uncertain {
            TriggerState::Unknown
        } else {
            TriggerState::Inactive
        };

        TriggerResult::new(state, matches)
    }

    fn unit(
        trigger: &EffectTrigger,
        entity: Option<EntityId>,
        game: &GameSnapshot,
    ) -> TriggerResult {
        let id = match trigger.unit {
            TriggerUnit::Player => game.player_id(),
            TriggerUnit::Target => game.target().entity_id(),
            TriggerUnit::Encounter => game
                .combat()
                .and_then(|combat| combat.encounter())
                .map(|encounter| encounter.entity_id()),
            _ => entity,
        };

        let Some(unit) = id.and_then(|id| game.entities().get(&id)) else {
            return TriggerResult::new(TriggerState::Unknown, Vec::new());
        };

        if !unit.present() {
            return TriggerResult::new(TriggerState::Unknown, Vec::new());
        }

        let mut groups: BTreeMap<(EffectId, Option<EntityId>), Vec<EffectSnapshot>> =
            BTreeMap::new();

        let mut uncertain = !unit.effects().complete();

        for effect in unit.effects().items() {
            if !trigger.effects.is_empty()
                && !trigger
                    .effects
                    .iter()
                    .any(|id| id.get() == effect.effect().effect_id().get())
            {
                continue;
            }

            if trigger.filter != EffectFilter::Any {
                let Some(kind) = effect.disposition() else {
                    uncertain = true;
                    continue;
                };

                let expected = match trigger.filter {
                    EffectFilter::Buff => EffectDisposition::Buff,
                    EffectFilter::Debuff => EffectDisposition::Debuff,
                    EffectFilter::Any => continue,
                };

                if kind != expected {
                    continue;
                }
            }

            if trigger.origin == EffectOrigin::Enemy {
                let Some(source) = effect.effect().source_id() else {
                    uncertain = true;
                    continue;
                };

                let Some(actor) = game.root_actor(source) else {
                    uncertain = true;
                    continue;
                };

                if game.group_member(actor) == Some(true) {
                    continue;
                }

                if !game
                    .combat()
                    .is_some_and(|combat| combat.opponents().contains(&actor))
                {
                    uncertain = true;
                    continue;
                }
            }

            if trigger.origin == EffectOrigin::Mine {
                let Some(player) = game.player_id() else {
                    uncertain = true;
                    continue;
                };

                let Some(source) = effect.effect().source_id() else {
                    uncertain = true;
                    continue;
                };

                let Some(actor) = game.root_actor(source) else {
                    uncertain = true;
                    continue;
                };

                if actor != player {
                    uncertain |= game.entities().get(&source).is_some_and(|unit| {
                        unit.player_summon() == Some(true) && unit.owner_id().is_none()
                    });
                    continue;
                }
            }

            groups
                .entry((effect.effect().effect_id(), effect.effect().source_id()))
                .or_default()
                .push(effect.clone());
        }

        let mut matches = Vec::new();

        for instances in groups.into_values() {
            if (instances.len() as f64) < trigger.min_stacks {
                continue;
            }

            if trigger.remaining_below_seconds > 0.0 {
                let earliest = instances
                    .iter()
                    .filter_map(EffectSnapshot::expires_at_us)
                    .min();

                let Some(end) = earliest else {
                    uncertain = true;
                    continue;
                };

                if end.saturating_sub(game.header().at_us()) as f64
                    > trigger.remaining_below_seconds * 1_000_000.0
                {
                    uncertain |= instances
                        .iter()
                        .any(|effect| effect.expires_at_us().is_none());

                    continue;
                }
            }

            matches.push(TriggerMatch::Effect {
                entity_id: unit.id(),
                instances,
            });
        }

        let present = if !matches.is_empty() {
            Some(true)
        } else if uncertain {
            None
        } else {
            Some(false)
        };

        let active =
            present.map(|present| present == (trigger.presence == EffectPresence::Present));

        TriggerResult::new(TriggerState::from_active(active, false), matches)
    }
}
