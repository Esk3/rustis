pub enum Ping {
    Pong,
    Null,
}
pub enum Echo {
    Echo(String),
    Null(String),
}
pub enum Get<V> {
    Value(V),
    NotFound,
    Null,
}
pub enum Set {
    Ok,
    NotAllowed,
    Null,
}
pub enum Wait {}
