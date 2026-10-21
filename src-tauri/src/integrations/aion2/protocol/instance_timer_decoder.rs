use super::BinaryReader;
use crate::core::game::GameEvent;
use crate::core::game::InstanceTimer;
use crate::core::game::MapId;

pub(super) struct InstanceTimerDecoder;

impl InstanceTimerDecoder {
    pub(super) fn decode(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let map_id = MapId::new(reader.u32()?)?;

        let kind = reader.byte()?;

        let started = u64::try_from(reader.i64()?).map_err(|_| "Negative timer start")?;

        let ends = u64::try_from(reader.i64()?).map_err(|_| "Negative timer end")?;

        if started < 1_600_000_000_000
            || (ends != 0 && (ends < started || ends - started > 86_400_000))
            || reader.remaining() != 0
        {
            return Err("Invalid instance timer".into());
        }

        Ok(vec![GameEvent::InstanceTimer {
            timer: InstanceTimer::new(map_id, kind, started, (ends != 0).then_some(ends)),
        }])
    }
}
