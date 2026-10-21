use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FieldBossDefinition {
    pub(crate) id: u32,
    pub(crate) npc_id: u32,
    pub(crate) map_id: u32,
}
