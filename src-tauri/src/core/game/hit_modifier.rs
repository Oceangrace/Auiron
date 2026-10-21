use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum HitModifier {
    Back,
    Frontal,
    Perfect,
    Double,
    Parry,
    ShieldBlock,
    IronWall,
    Regeneration,
    PerfectBlock,
}

impl Display for HitModifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Back => "back",
            Self::Frontal => "frontal",
            Self::Perfect => "perfect",
            Self::Double => "double",
            Self::Parry => "parry",
            Self::ShieldBlock => "shield-block",
            Self::IronWall => "iron-wall",
            Self::Regeneration => "regeneration",
            Self::PerfectBlock => "perfect-block",
        };

        formatter.write_str(value)
    }
}
