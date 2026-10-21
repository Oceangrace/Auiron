#[derive(Clone, Hash, Eq, PartialEq)]
pub(super) enum BindingKey {
    Primary(String),
    Rule { widget: String, rule: String },
}
