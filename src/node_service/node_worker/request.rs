pub struct Request {
    pub(super) id: usize,
    pub(super) kind: Kind,
}

pub enum Kind {
    Get { key: String },
}
