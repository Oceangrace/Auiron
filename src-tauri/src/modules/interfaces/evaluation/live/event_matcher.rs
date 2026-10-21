use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::GameSnapshot;
use crate::modules::interfaces::rules::EventParticipant;
use crate::modules::interfaces::rules::EventTrigger;

pub(super) struct EventMatcher;

impl EventMatcher {
    pub(super) fn supported(trigger: &EventTrigger, game: &GameSnapshot) -> bool {
        game.capabilities().supports(trigger.event) && trigger.schedule.is_none()
    }

    pub(super) fn ready(
        trigger: &EventTrigger,
        entity: Option<EntityId>,
        game: &GameSnapshot,
    ) -> bool {
        [trigger.source, trigger.recipient]
            .iter()
            .all(|participant| match participant {
                EventParticipant::Any => true,
                EventParticipant::Player => game.player_id().is_some(),
                EventParticipant::Party => game.party().is_some(),
                EventParticipant::Group => game.player_id().is_some() || game.party().is_some(),
                EventParticipant::Target => game.target().entity_id().is_some(),
                EventParticipant::Entity => entity.is_some(),
            })
    }

    pub(super) fn matches(
        event: &GameEvent,
        trigger: &EventTrigger,
        entity: Option<EntityId>,
        game: &GameSnapshot,
    ) -> Option<bool> {
        if event.event_type() != Some(trigger.event) {
            return Some(false);
        }

        if let Some(minimum) = trigger.min_value {
            let value = event.value()?;

            if f64::from(value) < minimum {
                return Some(false);
            }
        }

        if let Some(id) = trigger.skill_id
            && event.skill_id()?.get() != id.get()
        {
            return Some(false);
        }

        if let Some(id) = trigger.effect_id
            && event.effect_id()?.get() != id.get()
        {
            return Some(false);
        }

        let own_combo = matches!(
            event,
            GameEvent::ComboAvailable { .. } | GameEvent::ComboUnavailable { .. }
        );

        let source = if own_combo {
            game.player_id()
        } else {
            event.source()
        };

        let recipient = if own_combo {
            game.player_id()
        } else {
            event.recipient()
        };

        if !Self::participant(source, trigger.source, entity, game)?
            || !Self::participant(recipient, trigger.recipient, entity, game)?
        {
            return Some(false);
        }

        if let Some(npc) = trigger.source_npc_id {
            let actual = event
                .source()
                .and_then(|id| game.entities().get(&id))?
                .npc_id()?;

            return Some(actual.get() == npc.get());
        }

        Some(true)
    }

    fn participant(
        actual: Option<EntityId>,
        participant: EventParticipant,
        entity: Option<EntityId>,
        game: &GameSnapshot,
    ) -> Option<bool> {
        let expected = match participant {
            EventParticipant::Any => return Some(true),
            EventParticipant::Group | EventParticipant::Party => {
                let actor = game.root_actor(actual?)?;

                if participant == EventParticipant::Party && game.player_id() == Some(actor) {
                    return Some(false);
                }

                return game.group_member(actor);
            }

            EventParticipant::Player => game.player_id(),
            EventParticipant::Target => game.target().entity_id(),
            EventParticipant::Entity => entity,
        }?;

        Some(actual? == expected)
    }
}
