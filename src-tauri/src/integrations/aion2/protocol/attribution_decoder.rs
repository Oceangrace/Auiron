use super::BinaryReader;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;

pub(super) struct AttributionDecoder;

impl AttributionDecoder {
    pub(super) fn decode(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let entity_id = EntityId::new(reader.varint()?)?;

        reader.skip(4)?;

        let actor = reader.varint()?;

        if actor == 0 {
            return Ok(Vec::new());
        }

        let actor_id = EntityId::new(actor)?;

        if actor_id == entity_id || actor > 9_999_999 {
            return Err("Invalid attribution actor".into());
        }

        let mut events = vec![GameEvent::Attribution {
            entity_id,
            actor_id,
        }];

        if let Ok(identity) = Self::identity(reader, actor_id) {
            events.push(identity);
        }

        Ok(events)
    }

    fn identity(reader: &mut BinaryReader<'_>, entity_id: EntityId) -> Result<GameEvent, String> {
        let server = reader.u16()?;

        let length = reader.byte()?;

        if !(1000..=2999).contains(&server) || !(1..=48).contains(&length) {
            return Err("Unknown attribution profile".into());
        }

        let name = std::str::from_utf8(reader.bytes(usize::from(length))?)
            .map_err(|error| error.to_string())?;

        if !(1..=12).contains(&name.chars().count()) || !name.chars().all(char::is_alphanumeric) {
            return Err("Invalid attribution name".into());
        }

        Ok(GameEvent::Identity {
            entity_id,
            name: Some(name.to_owned()),
            own: false,
            server_id: Some(server),
            class_name: None,
            level: None,
        })
    }
}
