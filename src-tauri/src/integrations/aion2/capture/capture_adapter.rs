use std::ffi::CStr;
use std::ffi::CString;
use std::net::IpAddr;

pub(super) struct CaptureAdapter {
    name: CString,
    addresses: Vec<IpAddr>,
    loopback: bool,
}

impl CaptureAdapter {
    pub(super) fn new(name: CString, addresses: Vec<IpAddr>, loopback: bool) -> Self {
        Self {
            name,
            addresses,
            loopback,
        }
    }

    pub(super) fn name(&self) -> &CStr {
        &self.name
    }

    pub(super) fn accepts(&self, address: IpAddr) -> bool {
        self.addresses.contains(&address) || (self.loopback && address.is_loopback())
    }
}
