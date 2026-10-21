#[repr(C)]
pub(super) struct PcapAddress {
    pub(super) next: *mut PcapAddress,
    pub(super) address: *mut windows::Win32::Networking::WinSock::SOCKADDR,
    pub(super) netmask: *mut windows::Win32::Networking::WinSock::SOCKADDR,
    pub(super) broadcast: *mut windows::Win32::Networking::WinSock::SOCKADDR,
    pub(super) destination: *mut windows::Win32::Networking::WinSock::SOCKADDR,
}
