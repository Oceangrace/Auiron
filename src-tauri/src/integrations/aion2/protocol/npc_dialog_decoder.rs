use super::binary_reader::BinaryReader;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;

/// Диалог с NPC: `e241` открыт, `e243` закрыт.
pub(super) struct NpcDialogDecoder;

impl NpcDialogDecoder {
    /// `00 00`, `u32` сущность NPC, `u64` срок диалога (серверное время + 300 000 мс).
    pub(super) fn opened(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let entity_id = Self::entity(reader)?;

        reader.skip(8)?;

        Self::finish(reader)?;

        Ok(vec![GameEvent::NpcDialogOpened { entity_id }])
    }

    /// `00 00`, `u32` сущность NPC.
    pub(super) fn closed(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let entity_id = Self::entity(reader)?;

        Self::finish(reader)?;

        Ok(vec![GameEvent::NpcDialogClosed { entity_id }])
    }

    fn entity(reader: &mut BinaryReader<'_>) -> Result<EntityId, String> {
        if reader.bytes(2)? != [0, 0] {
            return Err("Unknown NPC dialog header".into());
        }

        EntityId::new(u64::from(reader.u32()?))
    }

    fn finish(reader: &BinaryReader<'_>) -> Result<(), String> {
        if reader.remaining() != 0 {
            return Err("Unknown NPC dialog suffix".into());
        }

        Ok(())
    }
}
