use crate::integrations::aion2::network::StreamKey;
use std::net::IpAddr;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct GameEndpoint {
    local_address: IpAddr,
    remote_address: IpAddr,
    local_port: u16,
    remote_port: u16,
}

impl GameEndpoint {
    pub(super) fn new(
        local_address: IpAddr,
        remote_address: IpAddr,
        local_port: u16,
        remote_port: u16,
    ) -> Self {
        Self {
            local_address,
            remote_address,
            local_port,
            remote_port,
        }
    }

    pub(super) fn local_address(&self) -> IpAddr {
        self.local_address
    }

    pub(super) fn same_server(&self, other: &Self) -> bool {
        self.local_address == other.local_address
            && self.remote_address == other.remote_address
            && self.remote_port == other.remote_port
    }

    pub(super) fn accepts(&self, stream: &StreamKey) -> bool {
        *stream
            == StreamKey::new(
                self.remote_address,
                self.local_address,
                self.remote_port,
                self.local_port,
            )
    }

    pub(super) fn filter(&self) -> String {
        let network = if self.local_address.is_ipv4() {
            "ip"
        } else {
            "ip6"
        };

        format!(
            "({network} and tcp and src host {} and src port {} and dst host {})",
            self.remote_address, self.remote_port, self.local_address
        )
    }
}
