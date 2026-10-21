use super::BinaryReader;
use super::damage_amount::DamageAmount;
use super::damage_hit_type::DamageHitType;
use super::damage_layout::DamageLayout;
use crate::core::game::EntityId;
use crate::core::game::SkillId;

/// One complete 0x3804 record. The body must end exactly after the optional suffixes.
pub(super) struct DamageRecord {
    recipient: EntityId,
    source: EntityId,
    skill: Option<SkillId>,
    hit_type: DamageHitType,
    amount: Option<DamageAmount>,
    effects: Vec<u32>,
}

impl DamageRecord {
    const EFFECTS: u64 = 0x01;
    const LIFE_STEAL: u64 = 0x04;
    const TRAILER: u64 = 0x10;
    const KNOWN_SWITCH: u64 = DamageLayout::PLOTTER
        | DamageLayout::VALUE
        | DamageLayout::FIELD
        | DamageLayout::ADDITIONAL_HITS
        | DamageLayout::PROTECTIONS;
    const KNOWN_EXTENSION: u64 = Self::EFFECTS | Self::LIFE_STEAL | Self::TRAILER;

    pub(super) fn read(reader: &mut BinaryReader<'_>) -> Result<Self, String> {
        let recipient = EntityId::new(reader.varint()?)?;

        let switch = reader.varint()?;

        let extension = reader.varint()?;

        let source = EntityId::new(reader.varint()?)?;

        let raw_skill = reader.u32()?;

        reader.byte()?;

        let hit_type = DamageHitType::from(reader.varint()?);

        if switch & !Self::KNOWN_SWITCH != 0 || extension & !Self::KNOWN_EXTENSION != 0 {
            return Err("Unknown damage record flags".into());
        }

        // A zero skill is only known as the notice of a reflected hit; its amount arrives in 0x3805.
        let skill = if raw_skill == 0
            && hit_type == DamageHitType::Reflection
            && switch == DamageLayout::VALUE
        {
            None
        } else {
            Some(SkillId::new(raw_skill)?)
        };

        let mut amount = if switch & DamageLayout::VALUE == 0 {
            Self::no_value(reader, switch)?;

            None
        } else {
            Some(DamageAmount::read(reader, switch)?)
        };

        let effects = if amount.is_none() && extension & Self::EFFECTS != 0 {
            DamageLayout::references(reader)?
        } else {
            Vec::new()
        };

        reader.u16()?;

        if extension & Self::LIFE_STEAL != 0 {
            amount
                .as_mut()
                .ok_or("Life steal without a damage value")?
                .read_life_steal(reader)?;
        }

        if extension & Self::TRAILER != 0
            && amount.as_ref().is_some_and(|amount| !amount.has_plotter())
        {
            reader.byte()?;
        }

        if reader.remaining() != 0 {
            return Err("Unrecognized damage record suffix".into());
        }

        Ok(Self {
            recipient,
            source,
            skill,
            hit_type,
            amount,
            effects,
        })
    }

    pub(super) fn recipient(&self) -> EntityId {
        self.recipient
    }

    pub(super) fn source(&self) -> EntityId {
        self.source
    }

    pub(super) fn skill(&self) -> Option<SkillId> {
        self.skill
    }

    pub(super) fn hit_type(&self) -> DamageHitType {
        self.hit_type
    }

    pub(super) fn effects(&self) -> &[u32] {
        &self.effects
    }

    pub(super) fn into_amount(self) -> Option<DamageAmount> {
        self.amount
    }

    fn no_value(reader: &mut BinaryReader<'_>, switch: u64) -> Result<(), String> {
        if switch != 0 {
            return Err("Unknown no-value damage switch".into());
        }

        reader.skip(8)?;
        reader.varint()?;

        Ok(())
    }
}
