use crate::integrations::aion2::GameId;
use crate::integrations::aion2::catalog::CatalogForm;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CatalogChoice {
    key: String,
    name: String,
    match_ids: Vec<GameId>,
}

impl CatalogChoice {
    pub(crate) fn form(&self) -> Result<CatalogForm, String> {
        if self.key.trim().is_empty() || self.name.trim().is_empty() || self.match_ids.is_empty() {
            return Err("Invalid catalog form".into());
        }

        Ok(CatalogForm {
            key: self.key.clone(),
            name: self.name.clone(),
            match_ids: self.match_ids.iter().map(|id| id.get()).collect(),
        })
    }
}
