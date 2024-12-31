use crate::connection::{ConnectionMessage, Output};

#[cfg(test)]
mod tests;

pub struct Follower;

impl Follower {
    pub fn new() -> Self {
        Self
    }

    fn handle_event(
        &mut self,
        event: crate::event::Kind,
    ) -> anyhow::Result<Option<ConnectionMessage>> {
        let res = match event {
            crate::event::Kind::Set { key, value, expiry } => {
                Some(ConnectionMessage::Input(crate::connection::Input::Set {
                    key,
                    value,
                    expiry: None,
                    get: false,
                }))
            }
        };
        Ok(res)
    }
}
