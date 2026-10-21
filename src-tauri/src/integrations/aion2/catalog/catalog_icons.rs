use super::CatalogItem;
use std::collections::HashMap;

#[derive(Default)]
pub(super) struct CatalogIcons {
    skills: HashMap<u32, Option<String>>,
    effects: HashMap<u32, Option<String>>,
}

impl CatalogIcons {
    pub(super) fn new<'a>(
        skills: impl Iterator<Item = &'a CatalogItem>,
        effects: impl Iterator<Item = &'a CatalogItem>,
    ) -> Self {
        Self {
            skills: Self::collect(skills),
            effects: Self::collect(effects),
        }
    }

    fn collect<'a>(items: impl Iterator<Item = &'a CatalogItem>) -> HashMap<u32, Option<String>> {
        let mut icons = HashMap::new();

        for item in items {
            let Some(icon) = &item.icon else {
                continue;
            };

            for id in item.forms.iter().flat_map(|form| &form.match_ids) {
                let candidate = Some(icon.clone());

                icons
                    .entry(*id)
                    .and_modify(|previous| {
                        if previous != &candidate {
                            *previous = None;
                        }
                    })
                    .or_insert(candidate);
            }
        }

        icons
    }

    pub(super) fn skill(&self, id: u32) -> Option<&str> {
        self.skills.get(&id).and_then(Option::as_deref)
    }

    pub(super) fn effect(&self, id: u32) -> Option<&str> {
        self.effects.get(&id).and_then(Option::as_deref)
    }
}
