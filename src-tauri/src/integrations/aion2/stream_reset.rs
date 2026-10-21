#[derive(Clone, Debug, PartialEq)]
pub(crate) enum StreamReset {
    Closed,
    Interrupted(String),
}
