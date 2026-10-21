use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UnitIdentity {
    character_id: Option<super::CharacterId>,
    name: Option<String>,
    class_name: Option<super::CharacterClass>,
    server_id: Option<u16>,
    level: Option<u32>,
}

impl UnitIdentity {
    pub(crate) fn observe(
        &mut self,
        name: &Option<String>,
        class_name: Option<super::CharacterClass>,
        server_id: Option<u16>,
        level: Option<u32>,
    ) {
        if let Some(name) = name {
            self.name = Some(name.clone());
        }

        self.class_name = class_name.or(self.class_name);
        self.server_id = server_id.or(self.server_id);
        self.level = level.or(self.level);
    }

    pub(crate) fn identify_character(&mut self, id: super::CharacterId) {
        self.character_id = Some(id);
        self.server_id = Some(id.server_id());
    }

    pub(crate) fn character_id(&self) -> Option<super::CharacterId> {
        self.character_id
    }

    pub(crate) fn same_character(&self, other: &Self) -> bool {
        if let Some((left, right)) = self.character_id.zip(other.character_id) {
            return left == right;
        }

        self.name
            .as_ref()
            .zip(self.server_id)
            .zip(other.name.as_ref().zip(other.server_id))
            .is_some_and(|(left, right)| left == right)
    }

    pub(crate) fn class_name(&self) -> Option<super::CharacterClass> {
        self.class_name
    }

    pub(crate) fn server_id(&self) -> Option<u16> {
        self.server_id
    }

    pub(crate) fn level(&self) -> Option<u32> {
        self.level
    }

    pub(crate) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
