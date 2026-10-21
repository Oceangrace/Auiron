use semver::Version;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReleaseInfo {
    current_version: String,
    latest_version: Option<String>,
    available: bool,
}

impl ReleaseInfo {
    pub(crate) fn new(current: &str, tag: Option<&str>) -> Result<Self, String> {
        let current = Version::parse(current).map_err(|error| error.to_string())?;

        let latest = tag
            .map(|tag| Version::parse(tag.strip_prefix('v').unwrap_or(tag)))
            .transpose()
            .map_err(|_| "Invalid release version".to_owned())?;

        let available = latest.as_ref().is_some_and(|version| {
            version.pre.is_empty() && version.cmp_precedence(&current).is_gt()
        });

        Ok(Self {
            current_version: current.to_string(),
            latest_version: latest.map(|version| version.to_string()),
            available,
        })
    }
}
