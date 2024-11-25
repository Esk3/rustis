pub struct Response {
    pub(super) id: usize,
    pub(super) kind: Kind,
}

pub enum Kind {
    Get { value: Option<String> },
    Set,
}
