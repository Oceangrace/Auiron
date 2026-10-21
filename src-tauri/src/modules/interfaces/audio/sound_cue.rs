use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum SoundCue {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "chime")]
    Chime,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "bell")]
    Bell,
    #[serde(rename = "double-bell")]
    DoubleBell,
    #[serde(rename = "crystal")]
    Crystal,
    #[serde(rename = "harp")]
    Harp,
    #[serde(rename = "marimba")]
    Marimba,
    #[serde(rename = "glass")]
    Glass,
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "pop")]
    Pop,
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "knock")]
    Knock,
    #[serde(rename = "rise")]
    Rise,
    #[serde(rename = "fall")]
    Fall,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "countdown")]
    Countdown,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "siren")]
    Siren,
    #[serde(rename = "radar")]
    Radar,
    #[serde(rename = "sonar")]
    Sonar,
    #[serde(rename = "horn")]
    Horn,
    #[serde(rename = "buzz")]
    Buzz,
    #[serde(rename = "heartbeat")]
    Heartbeat,
    #[serde(rename = "tick")]
    Tick,
    #[serde(rename = "speech")]
    Speech,
}
