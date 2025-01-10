use anyhow::anyhow;

use crate::resp::{self, value::IntoRespArray};

#[cfg(test)]
pub mod tests;

#[derive(Debug, PartialEq, Eq)]
pub struct OutgoingHandshake {
    advances: usize,
}

impl OutgoingHandshake {
    #[must_use]
    pub fn new() -> Self {
        Self { advances: 0 }
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.advances >= 5
    }

    pub fn try_advance(
        &mut self,
        response: &Option<Vec<resp::Value>>,
    ) -> anyhow::Result<Option<resp::Value>> {
        let result = match (self.advances, response) {
            (0, None) => Ok(Some(resp::Value::simple_string("PING"))),
            (1, Some(res)) if res.first().unwrap().eq_ignore_ascii_case("PONG") => Ok(Some(
                resp::Value::bulk_strings("REPLCONF; listing-port;1").into_array(),
            )),
            (2, Some(res)) if res.first().unwrap().eq_ignore_ascii_case("OK") => Ok(Some(
                resp::Value::bulk_strings("REPLCONF; CAPA; SYNC").into_array(),
            )),
            (3, Some(res)) if res.first().unwrap().eq_ignore_ascii_case("OK") => {
                Ok(Some(resp::Value::simple_string("PSYNC")))
            }
            (4, Some(res)) if res.first().unwrap().eq_ignore_ascii_case("PSYNC") => Ok(None),
            _ => Err(anyhow!("unepexted handshake message {response:?}")),
        };
        if result.is_ok() {
            self.advances += 1;
        } else {
            self.advances = 0;
        }
        result
    }
}
