use super::source::NpcCatalog;
use crate::core::game::NpcDefinitionId;
use std::collections::HashSet;

pub(crate) struct NpcIndex {
    ids: HashSet<NpcDefinitionId>,
}

impl NpcIndex {
    pub(crate) fn parse(bytes: &[u8], build: u32, version: &str) -> Result<Self, String> {
        let catalog = NpcCatalog::parse(bytes, build, version)?;
        let ids = catalog
            .ids()
            .iter()
            .map(|id| NpcDefinitionId::new(id.get()))
            .collect::<Result<_, _>>()?;

        Ok(Self { ids })
    }

    pub(crate) fn contains(&self, id: NpcDefinitionId) -> bool {
        self.ids.contains(&id)
    }
}
