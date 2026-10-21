pub(crate) trait TextMatcher: Send + Sync {
    fn contains(&self, locale: &str, text: &str, query: &str) -> Result<bool, String>;
}
