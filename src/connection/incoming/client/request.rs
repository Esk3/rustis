use crate::resp;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Request {
    pub value: Vec<resp::Value>,
    pub size: usize,
    pub timestamp: std::time::SystemTime,
}

impl Request {
    #[must_use]
    pub fn now(value: resp::Value, input_length: usize) -> Self {
        Self {
            value: value.into_array().unwrap(),
            size: input_length,
            timestamp: std::time::SystemTime::now(),
        }
    }
    #[allow(dead_code)]
    #[must_use]
    pub fn epoch(value: resp::Value, input_length: usize) -> Self {
        Self {
            value: value.into_array().unwrap(),
            size: input_length,
            timestamp: std::time::SystemTime::UNIX_EPOCH,
        }
    }
}
