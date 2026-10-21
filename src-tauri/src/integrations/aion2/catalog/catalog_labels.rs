use super::field_boss_definition::FieldBossDefinition;
use crate::core::game::EffectDisposition;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CatalogLabels {
    pub(crate) field_bosses: Vec<FieldBossDefinition>,
    pub(crate) automatic_skills: Vec<u32>,
    pub(crate) hidden_effects: Vec<u32>,
    pub(crate) skills: BTreeMap<u32, String>,
    pub(crate) npcs: BTreeMap<u32, String>,
    pub(crate) zones: BTreeMap<u32, String>,
    pub(crate) icons: BTreeMap<String, String>,
    pub(crate) skill_icons: BTreeMap<u32, String>,
    pub(crate) effect_icons: BTreeMap<u32, String>,
    pub(crate) effects: BTreeMap<u32, String>,
    pub(crate) effect_kinds: BTreeMap<u32, EffectDisposition>,
}
