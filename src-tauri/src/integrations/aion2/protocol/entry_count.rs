use super::BinaryReader;
use super::entry_value_mask::EntryValueMask;
use crate::core::game::OdyleEnergy;

pub(super) struct EntryCount {
    slot: u8,
    key: u64,
    basic: u32,
    charged: u32,
}

impl EntryCount {
    pub(super) fn read(reader: &mut BinaryReader<'_>) -> Result<Self, String> {
        let mask = EntryValueMask::try_from(reader.byte()?)?;

        let slot = reader.byte()?;

        let key = reader.varint()?;

        let first = u32::try_from(reader.varint()?).map_err(|_| "Invalid entry count")?;

        let (basic, charged) = match mask {
            EntryValueMask::Basic => (first, 0),
            EntryValueMask::Charged => (0, first),
            EntryValueMask::Both => (
                first,
                u32::try_from(reader.varint()?).map_err(|_| "Invalid charged count")?,
            ),
        };

        Ok(Self {
            slot,
            key,
            basic,
            charged,
        })
    }

    pub(super) fn follows(&self, previous: &Self) -> bool {
        self.slot == 2 && self.key == previous.key
    }

    pub(super) fn energy(&self) -> Option<OdyleEnergy> {
        (self.slot == 1 && self.basic <= 100_000 && self.charged <= 100_000)
            .then(|| OdyleEnergy::new(self.basic, self.charged))
    }
}
