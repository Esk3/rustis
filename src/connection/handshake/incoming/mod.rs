use anyhow::bail;

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

    pub fn try_advance(&mut self, input: &crate::Request) -> anyhow::Result<resp::Value> {
        let res = match (input.command().unwrap().to_uppercase().as_str(), self.count) {
            ("PING", 0) => {
                self.count = 1;
                "PONG"
            }
            ("REPLCONF", 0 | 1) => {
                self.count = 2;
                // TODO read port
                "OK"
            }
            ("REPLCONF", 2) => {
                self.count = 3;
                // TODO read capabilities
                "OK"
            }
            ("PSYNC", 3) => {
                self.count += 1;
                // TODO id & offset
                "FULLRESYNC 8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb 0"
            }
            _ => bail!("invalid advance {input:?}"),
        };
        Ok(resp::Value::simple_string(res))
    }
}
