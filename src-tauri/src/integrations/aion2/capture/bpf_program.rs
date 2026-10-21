#[repr(C)]
pub(super) struct BpfProgram {
    pub(super) count: u32,
    pub(super) instructions: *mut std::ffi::c_void,
}
