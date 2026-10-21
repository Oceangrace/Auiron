use super::BinaryReader;
use super::damage_hit_type::DamageHitType;
use super::damage_layout::DamageLayout;
use super::damage_plotter::DamagePlotter;
use crate::core::game::HitDetails;
use crate::core::game::HitOutcome;

pub(super) struct DamageAmount {
    plotter: Option<DamagePlotter>,
    scalar: u32,
    value: u32,
    additional: Vec<u32>,
    protections: Vec<u32>,
    life_steal: Option<u32>,
}

impl DamageAmount {
    pub(super) fn read(reader: &mut BinaryReader<'_>, switch: u64) -> Result<Self, String> {
        let plotter = if switch & DamageLayout::PLOTTER != 0 {
            Some(DamagePlotter::read(reader)?)
        } else {
            None
        };

        reader.skip(8)?;

        let scalar = Self::u32(reader.varint()?)?;

        let value = Self::u32(reader.varint()?)?;

        if switch & DamageLayout::FIELD != 0 && plotter.is_none() {
            reader.varint()?;
        }

        let additional = if switch & DamageLayout::ADDITIONAL_HITS != 0 {
            Self::additional(reader, value)?
        } else {
            Vec::new()
        };

        let protections = if switch & DamageLayout::PROTECTIONS != 0 {
            DamageLayout::references(reader)?
        } else {
            Vec::new()
        };

        Ok(Self {
            plotter,
            scalar,
            value,
            additional,
            protections,
            life_steal: None,
        })
    }

    pub(super) fn read_life_steal(&mut self, reader: &mut BinaryReader<'_>) -> Result<(), String> {
        self.life_steal = Some(Self::u32(reader.varint()?)?);

        Ok(())
    }

    pub(super) fn has_plotter(&self) -> bool {
        self.plotter.is_some()
    }

    pub(super) fn value(&self) -> u32 {
        self.value
    }

    pub(super) fn hit(self, hit_type: DamageHitType) -> HitDetails {
        let outcome = if hit_type == DamageHitType::Critical {
            HitOutcome::Critical
        } else {
            HitOutcome::Normal
        };

        let modifiers = self
            .plotter
            .as_ref()
            .map(DamagePlotter::modifiers)
            .unwrap_or_default();

        let restored = self.plotter.as_ref().map(DamagePlotter::restored);

        HitDetails::new(Some(outcome), modifiers, false, restored).amounts(
            (self.scalar != 0).then_some(self.scalar),
            &self.additional,
            self.life_steal,
            self.protections,
        )
    }

    fn additional(reader: &mut BinaryReader<'_>, value: u32) -> Result<Vec<u32>, String> {
        let count = reader.varint()?;

        if !(1..=25).contains(&count) {
            return Err("Invalid additional hit count".into());
        }

        let hits = (0..count)
            .map(|_| Self::u32(reader.varint()?))
            .collect::<Result<Vec<_>, _>>()?;

        if hits.iter().map(|hit| u64::from(*hit)).sum::<u64>() >= u64::from(value) {
            return Err("Invalid additional damage total".into());
        }

        Ok(hits)
    }

    fn u32(value: u64) -> Result<u32, String> {
        u32::try_from(value).map_err(|_| "Damage field exceeds u32".into())
    }
}
