#[derive(Clone)]
pub(crate) struct PreviewText {
    true_value: String,
    false_value: String,
    locale: String,
}

impl PreviewText {
    pub(crate) fn new(true_value: String, false_value: String, locale: String) -> Self {
        Self {
            true_value,
            false_value,
            locale,
        }
    }

    pub(crate) fn locale(&self) -> &str {
        &self.locale
    }

    pub(crate) fn boolean(&self, value: bool) -> &str {
        if value {
            return &self.true_value;
        }

        &self.false_value
    }
}

impl Default for PreviewText {
    fn default() -> Self {
        Self::new(true.to_string(), false.to_string(), String::new())
    }
}
