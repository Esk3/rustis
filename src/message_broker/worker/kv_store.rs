use crate::{
    message_broker::message::{self, Event},
    repository::Repository,
};

use super::Worker;

impl<R> Worker<R>
where
    R: Repository,
{
    pub fn handle_get(&mut self, key: &str) -> Result<Option<message::Response>, anyhow::Error> {
        self.repo
            .get(key)
            .map(|option| Some(message::Response::Get(option.cloned())))
    }
    pub fn handle_set(
        &mut self,
        key: String,
        value: String,
        expiry: Option<std::time::Duration>,
    ) -> Result<message::Response, anyhow::Error> {
        self.subscribers.notify(&Event::Set {
            key: key.clone(),
            value: value.clone(),
            expiry,
        });
        self.repo
            .set(key, value, expiry)
            .map(message::Response::Set)
    }
}
