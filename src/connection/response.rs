// TODO: either ResponseWraper struct or have response IntoFollower type variant
#[derive(Debug, PartialEq, Eq)]
pub enum Response {
    SendOk,
    SendBulkString(String),
    SendBytes(Vec<u8>),
    SendNull,
    None,
    SendPong,
}
