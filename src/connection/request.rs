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
}
