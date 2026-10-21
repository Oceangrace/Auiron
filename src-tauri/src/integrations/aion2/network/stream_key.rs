use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) struct StreamKey {
    source: IpAddr,
    destination: IpAddr,
    source_port: u16,
    destination_port: u16,
}

impl StreamKey {
    pub(crate) fn new(
        source: IpAddr,
        destination: IpAddr,
        source_port: u16,
        destination_port: u16,
    ) -> Self {
        Self {
            source,
            destination,
            source_port,
            destination_port,
        }
    }
}
