use super::IconResponse;

pub(crate) trait IconTransport: Send + Sync {
    fn fetch(&self, name: &str) -> IconResponse;
}
