use super::CapturedPacket;

pub(crate) enum CaptureRead {
    Packet(CapturedPacket),
    Reset,
    Pending,
    Finished,
}
