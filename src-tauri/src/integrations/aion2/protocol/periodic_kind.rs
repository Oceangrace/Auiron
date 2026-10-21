/// Verified 0x3805 kinds. Kind 9 is only used as a forecast pool for explicitly verified skill
/// forms. It never counts as a healing hit; unverified skill forms remain ignored.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PeriodicKind {
    Healing,
    Damage,
    EffectDamage,
    PeriodicHealing,
    HealingPool,
    ReflectedDamage,
}

impl PeriodicKind {
    pub(super) fn parse(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Healing),
            2 => Some(Self::Damage),
            10 => Some(Self::EffectDamage),
            11 => Some(Self::PeriodicHealing),
            9 => Some(Self::HealingPool),
            74 => Some(Self::ReflectedDamage),
            _ => None,
        }
    }

    pub(super) fn healing(self) -> bool {
        matches!(self, Self::Healing | Self::PeriodicHealing)
    }

    pub(super) fn periodic(self) -> bool {
        matches!(
            self,
            Self::Damage | Self::EffectDamage | Self::PeriodicHealing
        )
    }
}
