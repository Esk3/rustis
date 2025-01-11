use std::collections::HashMap;

use anyhow::{bail, Context};

use crate::resp;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Parser {
    value: std::vec::IntoIter<resp::Value>,
    store: HashMap<String, String>,
}

impl Parser {
    pub fn new(value: Vec<resp::Value>) -> Self {
        Self {
            value: value.into_iter(),
            store: HashMap::new(),
        }
    }

    pub fn ident(mut self, ident: &str) -> anyhow::Result<Self> {
        if !self
            .value
            .next()
            .context("no more values")?
            .eq_ignore_ascii_case(ident)
        {
            bail!("ident: {ident} not found");
        }
        Ok(self)
    }
    pub fn value(mut self, key: impl ToString) -> anyhow::Result<Self> {
        let value = self
            .value
            .next()
            .context("no more values")?
            .expect_string()?;
        self.store.insert(key.to_string(), value);
        Ok(self)
    }
    pub fn finish(self) -> HashMap<String, String> {
        self.store
    }
}
