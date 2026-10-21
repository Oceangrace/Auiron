use super::GameSnapshot;

#[derive(Clone)]
pub(crate) struct SourceFrame {
    snapshot: GameSnapshot,
    error: Option<String>,
}

impl SourceFrame {
    pub(crate) fn new(snapshot: GameSnapshot, error: Option<String>) -> Self {
        Self { snapshot, error }
    }

    pub(crate) fn snapshot(&self) -> &GameSnapshot {
        &self.snapshot
    }

    pub(crate) fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}
