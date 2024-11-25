#[derive(Debug, PartialEq, Eq)]
pub enum Response {
    Ok,
    Something(()),
    Null,
    None,
    Pong,
}
