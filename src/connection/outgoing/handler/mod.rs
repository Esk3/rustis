use crate::connection::{Input, Output};

#[cfg(test)]
mod tests;

pub struct Handler;

impl Handler {
    pub fn new() -> Self {
        Self
    }

    fn handle_request(&mut self, ping: Input) -> anyhow::Result<Option<Output>> {
        Ok(None)
    }
}
