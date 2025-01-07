use std::fmt::Debug;

use crate::{
    config::RedisConfig, connection::Connection, event::EventEmitter, listner::RedisListner,
    repository::Repository,
};
use anyhow::Context;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct RedisBuilder<L, C> {
    config: Option<RedisConfig>,
    listner: Option<L>,
    leader_connection: Option<C>,
    repo: Option<Repository>,
    emitter: Option<EventEmitter>,
}

impl<L, C> RedisBuilder<L, C>
where
    L: RedisListner,
    C: Connection,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: None,
            listner: None,
            leader_connection: None,
            repo: None,
            emitter: None,
        }
    }

    pub fn build(self) -> anyhow::Result<super::Redis<L, C>> {
        Ok(super::Redis::new(
            self.listner.context("listner missing")?,
            self.leader_connection,
            self.repo.context("repo missing")?,
            self.emitter.context("emitter missing")?,
        ))
    }

    pub fn bind(self, port: u16) -> anyhow::Result<Self> {
        if self.listner.is_some() {
            tracing::warn!("listner overwritten");
        }
        let listner = L::bind(port)?;
        Ok(Self {
            listner: Some(listner),
            ..self
        })
    }

    #[must_use]
    pub fn listner(self, listner: L) -> Self {
        if self.listner.is_some() {
            tracing::warn!("listner overwritten");
        }
        Self {
            listner: Some(listner),
            ..self
        }
    }

    #[must_use]
    pub fn repo(self, repo: Repository) -> Self {
        if self.repo.is_some() {
            tracing::warn!("repo overwritten");
        }
        Self {
            repo: Some(repo),
            ..self
        }
    }
    #[must_use]
    pub fn emitter(self, emitter: EventEmitter) -> Self {
        if self.emitter.is_some() {
            tracing::warn!("emitter overwritten");
        }
        Self {
            emitter: Some(emitter),
            ..self
        }
    }

    #[must_use]
    pub fn leader_addr(self) -> Self {
        todo!()
    }

    #[must_use]
    pub fn leader_connection(self, connection: C) -> Self {
        if self.leader_connection.is_some() {
            tracing::warn!("leader connection overwritten");
        }
        Self {
            leader_connection: Some(connection),
            ..self
        }
    }
}
