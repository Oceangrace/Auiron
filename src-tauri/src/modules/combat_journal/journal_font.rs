use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize, schemars::JsonSchema)]
pub(super) enum JournalFont {
    #[serde(rename = "lora")]
    Lora,
    #[serde(rename = "sans")]
    Sans,
    #[serde(rename = "mono")]
    Mono,
    #[default]
    #[serde(rename = "inter")]
    Inter,
    #[serde(rename = "roboto")]
    Roboto,
    #[serde(rename = "open-sans")]
    OpenSans,
    #[serde(rename = "noto-sans")]
    NotoSans,
    #[serde(rename = "source-sans-3")]
    SourceSans,
    #[serde(rename = "ubuntu-sans")]
    UbuntuSans,
    #[serde(rename = "montserrat")]
    Montserrat,
    #[serde(rename = "roboto-condensed")]
    RobotoCondensed,
    #[serde(rename = "oswald")]
    Oswald,
    #[serde(rename = "ibm-plex-sans")]
    IbmPlexSans,
    #[serde(rename = "roboto-mono")]
    RobotoMono,
}
