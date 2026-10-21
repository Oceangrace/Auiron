pub(crate) enum IconResponse {
    Image(Vec<u8>),
    Missing,
    Retry(String),
    Offline(String),
}
