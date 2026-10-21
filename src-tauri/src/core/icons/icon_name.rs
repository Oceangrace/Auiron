pub(crate) struct IconName;

impl IconName {
    pub(crate) fn file(path: &str) -> Result<String, String> {
        if path.len() > 220 || path.split('/').any(|part| !Self::component(part)) {
            return Err(format!("Invalid catalog icon path: {path}"));
        }

        Ok(format!(
            "{}.png",
            path.replace('/', "_").to_ascii_lowercase()
        ))
    }

    pub(crate) fn component(value: &str) -> bool {
        !value.is_empty()
            && value.len() <= 220
            && value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    }

    pub(crate) fn valid_file(file: &str) -> bool {
        file.strip_suffix(".png").is_some_and(|stem| {
            Self::component(stem) && !stem.bytes().any(|byte| byte.is_ascii_uppercase())
        })
    }
}
