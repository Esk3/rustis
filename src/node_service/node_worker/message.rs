#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    pub(super) id: usize,
    pub(super) kind: Kind,
}

#[derive(Debug)]
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
    ToFollower,
    ToFollowerOk,
    Wait {
        count: usize,
    },
    WaitResponse {
        count: usize,
    },
    SyncBytesSent,
    SyncBytesSentAck,
    WaitTimeout,
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Get { key: l_key }, Self::Get { key: r_key }) => l_key == r_key,
            (Self::GetResponse { value: l_value }, Self::GetResponse { value: r_value }) => {
                l_value == r_value
            }
            (
                Self::Set {
                    key: l_key,
                    value: l_value,
                    expiry: l_expiry,
                },
                Self::Set {
                    key: r_key,
                    value: r_value,
                    expiry: r_expiry,
                },
            ) => l_key == r_key && l_value == r_value && l_expiry == r_expiry,
            (
                Self::ReplicateSet {
                    key: l_key,
                    value: l_value,
                    expiry: l_expiry,
                },
                Self::ReplicateSet {
                    key: r_key,
                    value: r_value,
                    expiry: r_expiry,
                },
            ) => l_key == r_key && l_value == r_value && l_expiry == r_expiry,
            (Self::NewConnection { tx: l_tx }, Self::NewConnection { tx: r_tx }) => true,
            (
                Self::NewConnectionResponse { id: l_id },
                Self::NewConnectionResponse { id: r_id },
            ) => l_id == r_id,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for Kind {}
