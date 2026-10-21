use super::BinaryReader;
use crate::core::game::GameEvent;
use crate::core::game::MapId;
use crate::core::game::WorldBoss;
use crate::core::game::WorldBossList;
use std::collections::BTreeSet;

pub(super) struct WorldBossDecoder;

impl WorldBossDecoder {
    pub(super) fn decode(
        reader: &mut BinaryReader<'_>,
        at_us: u64,
    ) -> Result<Vec<GameEvent>, String> {
        if reader.u16()? != 0 {
            return Err("Unsupported world boss map header".into());
        }

        let map_id = MapId::new(reader.u32()?)?;

        let count = reader.varint()?;

        if !(1..=200).contains(&count) {
            return Err("Invalid world boss count".into());
        }

        let mut bosses = Vec::new();

        let mut ids = BTreeSet::new();

        for index in 0..count {
            let alive = match reader.byte()? {
                0 => false,
                1 => true,
                _ => return Err("Invalid world boss presence".into()),
            };

            let id = u32::try_from(reader.varint()?).map_err(|_| "Invalid world boss ID")?;

            if !(1_000..10_000_000).contains(&id) || !ids.insert(id) {
                return Err("Invalid or duplicate world boss ID".into());
            }

            if alive {
                for _ in 0..3 {
                    if !f32::from_bits(reader.u32()?).is_finite() {
                        return Err("Invalid world boss position".into());
                    }
                }
            }

            let (time, bytes) = Self::timestamp(reader, index + 1 == count, at_us / 1000)?;

            reader.skip(bytes)?;
            bosses.push(WorldBoss::new(id, alive, (time != 0).then_some(time)));
        }

        if !Self::ending(reader) {
            return Err("Unknown world boss suffix".into());
        }

        Ok(vec![GameEvent::WorldBossList {
            list: WorldBossList::new(map_id, at_us, bosses),
        }])
    }

    fn timestamp(
        reader: &BinaryReader<'_>,
        last: bool,
        now_ms: u64,
    ) -> Result<(u64, usize), String> {
        let mut accepted = None;

        for extra in 0..=1 {
            let mut candidate = reader.fork();

            if candidate.skip(extra).is_err() {
                continue;
            }

            let Ok(raw) = candidate.i64() else {
                continue;
            };

            let Ok(time) = u64::try_from(raw) else {
                continue;
            };

            let plausible = time == 0
                || (now_ms.saturating_sub(14 * 86_400_000)..=now_ms.saturating_add(3 * 86_400_000))
                    .contains(&time);

            if !plausible || !Self::next_entry(&candidate, last) {
                continue;
            }

            if accepted.is_some() {
                return Err("Ambiguous world boss time layout".into());
            }

            accepted = Some((time, extra + 8));
        }

        accepted.ok_or_else(|| "Unknown world boss time layout".into())
    }

    fn next_entry(reader: &BinaryReader<'_>, last: bool) -> bool {
        if last {
            return Self::ending(reader);
        }

        let mut candidate = reader.fork();

        matches!(candidate.byte(), Ok(0 | 1))
            && candidate
                .varint()
                .is_ok_and(|id| (1_000..10_000_000).contains(&id))
    }

    fn ending(reader: &BinaryReader<'_>) -> bool {
        let mut candidate = reader.fork();

        candidate.remaining() == 0
            || (candidate.remaining() == 3
                && candidate.bytes(3).is_ok_and(|bytes| bytes == [0, 0, 0]))
    }
}
