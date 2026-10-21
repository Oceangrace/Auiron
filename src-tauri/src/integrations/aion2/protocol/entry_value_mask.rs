pub(super) enum EntryValueMask {
    Basic,
    Charged,
    Both,
}

impl TryFrom<u8> for EntryValueMask {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            4 => Ok(Self::Basic),
            8 => Ok(Self::Charged),
            12 => Ok(Self::Both),
            _ => Err("Unknown entry value mask".into()),
        }
    }
}
