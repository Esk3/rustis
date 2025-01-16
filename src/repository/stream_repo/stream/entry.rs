use crate::resp::value::IntoRespArray;
use crate::{
    repository::stream_repo::{stream::Field, EntryId},
    resp,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub(super) id: EntryId,
    pub(super) fields: Vec<Field>,
}

impl Entry {
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn new(id: EntryId, fields: Vec<Field>) -> Self {
        Self { id, fields }
    }

    #[must_use]
    pub fn id(&self) -> &EntryId {
        &self.id
    }

    #[must_use]
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}

impl From<Entry> for resp::Value {
    fn from(value: Entry) -> Self {
        [
            resp::Value::simple_string(value.id),
            value
                .fields
                .into_iter()
                .flat_map(|field| [field.name, field.value])
                .map(resp::Value::simple_string)
                .collect(),
        ]
        .into_array()
    }
}
