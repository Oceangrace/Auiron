use super::BinaryReader;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::ServerBattle;

pub(super) struct BattleDecoder;

impl BattleDecoder {
    pub(super) fn decode(
        reader: &mut BinaryReader<'_>,
        at_us: u64,
    ) -> Result<Vec<GameEvent>, String> {
        let entity_id = EntityId::new(reader.varint()?)?;

        reader.varint()?;

        let active = match reader.varint()? {
            0 => false,
            1 => true,
            _ => return Err("Unknown server battle flag".into()),
        };

        let deadline_ms = if active && reader.remaining() == 8 {
            let time = u64::try_from(reader.i64()?).map_err(|_| "Negative battle deadline")?;

            let now = at_us / 1000;

            (time > now && time - now <= 3_600_000).then_some(time)
        } else {
            None
        };

        if reader.remaining() != 0 {
            return Err("Unknown server battle suffix".into());
        }

        Ok(vec![GameEvent::ServerBattleChanged {
            entity_id,
            battle: ServerBattle::new(active, deadline_ms),
        }])
    }
}
