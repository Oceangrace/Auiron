#[repr(C)]
pub(super) struct PcapStatistics {
    pub(super) received: u32,
    pub(super) dropped: u32,
    pub(super) interface_dropped: u32,
    pub(super) captured: u32,
}
