use anyhow::{anyhow, bail};

use crate::resp;

#[cfg(test)]
pub mod tests;

pub struct IncomingHandshake {
    count: usize,
}
impl IncomingHandshake {
    #[must_use]
    pub fn new() -> Self {
        Self { count: 0 }
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.count >= 4
    }

    pub fn try_advance(&mut self, input: &[resp::Value]) -> anyhow::Result<resp::Value> {
        let res = match (
            input
                .first()
                .unwrap()
                .clone()
                .expect_string()
                .unwrap()
                .to_uppercase()
                .as_str(),
            self.count,
        ) {
            ("PING", 0) => {
                self.count = 1;
                "PONG"
            }
            ("REPLCONF", 0 | 1) => {
                self.count = 2;
                "OK"
            }
            ("REPLCONF", 2) => {
                self.count = 3;
                "OK"
            }
            ("PSYNC", 3) => {
                self.count += 1;
                "PSYNC"
            }
            _ => bail!("invalid advance {input:?}"),
        };
        Ok(resp::Value::simple_string(res))
    }
}
