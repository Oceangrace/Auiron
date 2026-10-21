use super::binary_reader::BinaryReader;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;

/// The local client's target notification; other actors' targets use a different message.
pub(super) struct TargetDecoder;

impl TargetDecoder {
    pub(super) fn decode(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        if reader.bytes(3)? != [0, 0, 0] {
            return Err("Unknown target notification header".into());
        }

        let id = reader.varint()?;

        if reader.remaining() != 0 {
            return Err("Unknown target notification suffix".into());
        }

        let entity_id = match id {
            0 => None,
            id => Some(EntityId::new(id)?),
        };

        Ok(vec![GameEvent::TargetSelected { entity_id }])
    }
}
