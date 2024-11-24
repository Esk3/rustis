#[derive(Debug, PartialEq, Eq)]
pub enum Ping {
    Pong,
    Null,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Echo {
    Echo(String),
    Null(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Get<V> {
    Value(V),
    NotFound,
    Null,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Set {
    Ok,
    NotAllowed,
    Null,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Wait {}
