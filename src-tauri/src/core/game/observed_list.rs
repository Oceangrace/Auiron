use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", content = "items", rename_all = "kebab-case")]
pub(crate) enum ObservedList<T> {
    Unknown,
    Partial(Vec<T>),
    Complete(Vec<T>),
}

impl<T> ObservedList<T> {
    pub(crate) fn items(&self) -> &[T] {
        match self {
            Self::Unknown => &[],
            Self::Partial(items) | Self::Complete(items) => items,
        }
    }

    pub(crate) fn complete(&self) -> bool {
        matches!(self, Self::Complete(_))
    }
}
