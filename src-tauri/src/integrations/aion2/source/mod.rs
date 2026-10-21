//! Source lifetime and scheduling; packet handles remain on their owning worker thread.

mod game_source;
mod session_checkpoint;
mod source_reader;
mod source_recovery;
mod source_request;
mod source_worker;

pub(crate) use self::game_source::GameSource;
