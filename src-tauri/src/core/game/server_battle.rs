use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ServerBattle {
    active: bool,
    deadline_ms: Option<u64>,
}

impl ServerBattle {
    pub(crate) fn new(active: bool, deadline_ms: Option<u64>) -> Self {
        Self {
            active,
            deadline_ms,
        }
    }

    pub(crate) fn active(&self) -> bool {
        self.active
    }

    pub(crate) fn deadline_ms(&self) -> Option<u64> {
        self.deadline_ms
    }
}
