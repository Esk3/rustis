pub struct Message {
    pub(super) id: usize,
    pub(super) kind: Kind,
}

pub enum Kind {
    Get {
        key: String,
    },
    GetResponse {
        value: Option<String>,
    },
    Set {
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
    },
    ReplicateSet {
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
    },
    SetResponse,
    NewConnection {
        tx: std::sync::mpsc::Sender<Message>,
    },
    NewConnectionResponse {
        id: usize,
    },
}
