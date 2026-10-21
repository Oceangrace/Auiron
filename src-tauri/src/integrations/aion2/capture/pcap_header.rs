#[repr(C)]
pub(super) struct PcapHeader {
    pub(super) seconds: i32,
    pub(super) microseconds: i32,
    pub(super) captured: u32,
    pub(super) original: u32,
}
