use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct GitHubRelease {
    tag_name: String,
    draft: bool,
    prerelease: bool,
}

impl GitHubRelease {
    pub(super) fn version(&self) -> Option<&str> {
        (!self.draft && !self.prerelease).then_some(self.tag_name.as_str())
    }
}
