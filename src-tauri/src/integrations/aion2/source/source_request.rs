use crate::config::CaptureConfiguration;
use crate::integrations::aion2::capture::CaptureTransport;
use crate::integrations::aion2::capture::LiveTransport;
use crate::integrations::aion2::capture::ReplayTransport;
use std::path::PathBuf;

pub(super) enum SourceRequest {
    Live,
    Replay {
        path: PathBuf,
        speed: f64,
        ports: Vec<u16>,
    },
}

impl SourceRequest {
    pub(super) fn environment() -> Result<Self, String> {
        let Some(path) = std::env::var_os("AURION_AION2_REPLAY_FILE") else {
            return Ok(Self::Live);
        };

        let speed = std::env::var("AURION_AION2_REPLAY_SPEED")
            .unwrap_or_else(|_| "1".into())
            .parse::<f64>()
            .map_err(|error| error.to_string())?;

        let ports = std::env::var("AURION_AION2_REPLAY_SERVER_PORTS")
            .unwrap_or_else(|_| "1111".into())
            .split(',')
            .map(|value| {
                value
                    .trim()
                    .parse::<u16>()
                    .map_err(|error| error.to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::Replay {
            path: path.into(),
            speed,
            ports,
        })
    }

    pub(super) fn transport(
        self,
        configuration: CaptureConfiguration,
    ) -> Result<Box<dyn CaptureTransport>, String> {
        match self {
            Self::Live => Ok(Box::new(LiveTransport::new(configuration))),
            Self::Replay { path, speed, ports } => {
                Ok(Box::new(ReplayTransport::new(path, speed, ports)?))
            }
        }
    }
}
