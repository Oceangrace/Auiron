use super::trigger_match::TriggerMatch;
use super::trigger_result::TriggerResult;
use super::trigger_state::TriggerState;
use crate::core::game::GameSnapshot;
use crate::modules::interfaces::rules::ObservationKind;

pub(super) struct CooldownMatcher;

impl CooldownMatcher {
    pub(super) fn evaluate(
        observation: &ObservationKind,
        ids: &[u32],
        game: &GameSnapshot,
        resolver: &super::selection_resolver::SelectionResolver<'_>,
    ) -> TriggerResult {
        if *observation == ObservationKind::SkillState {
            let Some(player) = game.player_id() else {
                return TriggerResult::new(TriggerState::Unknown, Vec::new());
            };

            if !game
                .entities()
                .get(&player)
                .is_some_and(|unit| unit.present())
            {
                return TriggerResult::new(TriggerState::Unknown, Vec::new());
            }

            let matches = resolver
                .cooldown(ids, game)
                .filter(|cooldown| cooldown.remaining_ms() > 0)
                .map(|cooldown| TriggerMatch::Cooldown {
                    cooldown: cooldown.clone(),
                })
                .into_iter()
                .collect();

            return TriggerResult::new(TriggerState::Active, matches);
        }

        if *observation == ObservationKind::ComboAvailable {
            let active = resolver.combo_available(ids, game);

            return TriggerResult::new(TriggerState::from_active(active, false), Vec::new());
        }

        let Some(cooldown) = resolver.cooldown(ids, game) else {
            return TriggerResult::new(TriggerState::Unknown, Vec::new());
        };

        let active = match observation {
            ObservationKind::Cooldown => Some(cooldown.remaining_ms() > 0),
            ObservationKind::SkillReady => cooldown.ready(),
            _ => None,
        };

        TriggerResult::new(
            TriggerState::from_active(active, false),
            vec![TriggerMatch::Cooldown {
                cooldown: cooldown.clone(),
            }],
        )
    }
}
