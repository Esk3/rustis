use anyhow::anyhow;

use crate::resp::{Input, Output, ReplConf};

#[cfg(test)]
pub mod tests;

pub struct IncomingHandshake {
    finished: usize,
}
impl IncomingHandshake {
    pub fn new() -> Self {
        Self { finished: 0 }
    }

    pub fn is_finished(&self) -> bool {
        self.finished >= 4
    }

    pub fn try_advance(&mut self, input: &Input) -> anyhow::Result<Output> {
        let res = match input {
            Input::Ping => Ok(Output::Pong),
            Input::ReplConf(ReplConf::ListingPort(_)) => Ok(ReplConf::Ok.into()),
            Input::ReplConf(ReplConf::Capa(_)) => Ok(ReplConf::Ok.into()),
            Input::Psync => Ok(Output::Psync),
            _ => Err(anyhow!("invalid start")),
        };
        self.finished += 1;
        res
    }
}
