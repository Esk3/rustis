#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub(super) name: String,
    pub(super) value: String,
}

impl Field {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(name: impl ToString, value: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}
