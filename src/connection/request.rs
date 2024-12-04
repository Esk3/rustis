#[derive(Debug, PartialEq, Eq)]
pub enum Request {
    Ping,
    Echo(String),
    Get(String),
    Set {
        key: String,
        value: String,
        exp: Option<std::time::Duration>,
    },
    Info,
    Sync,
    // don't know if this should be implicit with sync or if i inject this somewhere
    IntoFollower,
    Wait,
    Multi,
    AbortQueue,
    ExecuteQueue,
    StreamAdd,
    StreamGet,
    StreamQuery,
}
