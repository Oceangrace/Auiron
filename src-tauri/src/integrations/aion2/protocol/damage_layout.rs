use super::BinaryReader;

pub(super) struct DamageLayout;

impl DamageLayout {
    pub(super) const PLOTTER: u64 = 0x02;
    pub(super) const VALUE: u64 = 0x04;
    pub(super) const FIELD: u64 = 0x10;
    pub(super) const ADDITIONAL_HITS: u64 = 0x20;
    pub(super) const PROTECTIONS: u64 = 0x40;

    pub(super) fn references(reader: &mut BinaryReader<'_>) -> Result<Vec<u32>, String> {
        let count = reader.varint()?;

        if !(1..=16).contains(&count) {
            return Err("Invalid damage reference count".into());
        }

        (0..count).map(|_| reader.u32()).collect()
    }
}
