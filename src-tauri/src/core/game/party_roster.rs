use super::PartyMember;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PartyRoster {
    members: Vec<PartyMember>,
    complete: bool,
    dungeon_id: Option<u32>,
}

impl PartyRoster {
    pub(crate) fn new(members: Vec<PartyMember>, complete: bool, dungeon_id: Option<u32>) -> Self {
        Self {
            members,
            complete,
            dungeon_id,
        }
    }

    pub(crate) fn members(&self) -> &[PartyMember] {
        &self.members
    }

    pub(crate) fn complete(&self) -> bool {
        self.complete
    }

    pub(crate) fn dungeon_id(&self) -> Option<u32> {
        self.dungeon_id
    }
}
