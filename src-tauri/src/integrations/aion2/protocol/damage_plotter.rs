use super::BinaryReader;
use crate::core::game::HitModifier;

pub(super) struct DamagePlotter {
    flags: u8,
    restored: u32,
    angle: u8,
}

impl DamagePlotter {
    pub(super) fn read(reader: &mut BinaryReader<'_>) -> Result<Self, String> {
        let flags = reader.byte()?;

        let restored = u32::try_from(reader.varint()?).map_err(|_| "Invalid restored HP")?;

        let angle = reader.byte()?;

        Ok(Self {
            flags,
            restored,
            angle,
        })
    }

    pub(super) fn restored(&self) -> u32 {
        self.restored
    }

    pub(super) fn modifiers(&self) -> Vec<HitModifier> {
        let mut modifiers: Vec<_> = [
            (1, HitModifier::ShieldBlock),
            (2, HitModifier::Parry),
            (4, HitModifier::Perfect),
            (8, HitModifier::Double),
            (16, HitModifier::IronWall),
            (32, HitModifier::Regeneration),
            (64, HitModifier::PerfectBlock),
        ]
        .into_iter()
        .filter(|(bit, _)| self.flags & bit != 0)
        .map(|(_, modifier)| modifier)
        .collect();

        match self.angle {
            1 => modifiers.push(HitModifier::Back),
            2 => modifiers.push(HitModifier::Frontal),
            _ => {}
        }

        modifiers
    }
}
