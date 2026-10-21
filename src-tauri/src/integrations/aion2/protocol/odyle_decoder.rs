use super::BinaryReader;
use super::entry_count::EntryCount;
use crate::core::game::GameEvent;
use crate::core::game::OdyleEnergy;

pub(super) struct OdyleDecoder;

impl OdyleDecoder {
    pub(super) fn update(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let count = reader.byte()?.max(1);

        let mut energy = None;

        for _ in 0..count {
            let entry = EntryCount::read(reader)?;

            if let Some(value) = entry.energy() {
                if energy.is_some() {
                    return Err("Duplicate Odyle energy entry".into());
                }

                energy = Some(value);
            }
        }

        Ok(Self::events(energy))
    }

    pub(super) fn list(reader: &mut BinaryReader<'_>) -> Result<Vec<GameEvent>, String> {
        let body = reader.bytes(reader.remaining())?;

        if body.len() > 65_536 {
            return Err("Entry list exceeds the inspection limit".into());
        }

        let mut energy = None;

        for position in 0..body.len().saturating_sub(1) {
            if body[position + 1] != 1 || !matches!(body[position], 4 | 8 | 12) {
                continue;
            }

            let mut candidate = BinaryReader::new(&body[position..]);

            let Ok(first) = EntryCount::read(&mut candidate) else {
                continue;
            };

            let Ok(next) = EntryCount::read(&mut candidate) else {
                continue;
            };

            if next.follows(&first)
                && let Some(value) = first.energy()
            {
                if energy.is_some() {
                    return Err("Ambiguous Odyle energy list".into());
                }

                energy = Some(value);
            }
        }

        Ok(Self::events(energy))
    }

    fn events(energy: Option<OdyleEnergy>) -> Vec<GameEvent> {
        energy
            .map(|energy| GameEvent::OdyleEnergy { energy })
            .into_iter()
            .collect()
    }
}
