pub struct Request {
    pub(super) id: usize,
    pub(super) kind: Kind,
}

pub enum Kind {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
    },
    NewConnection {
        tx: std::sync::mpsc::Sender<super::Response>,
    },
}
