use crate::{
    event::Kind,
    resp::{self, value::IntoRespArray},
};

//#[cfg(test)]
//mod tests;

pub struct Follower;

impl Follower {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_event(&mut self, event: Kind) -> anyhow::Result<Option<resp::Value>> {
        let res = match event {
            Kind::Set {
                key,
                value,
                expiry: _,
            } => Some(
                vec![
                    resp::Value::bulk_string("SET"),
                    resp::Value::bulk_string(key),
                    resp::Value::bulk_string(value),
                ]
                .into_array(),
            ),
        };
        Ok(res)
    }
}
