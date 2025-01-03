use anyhow::bail;

use crate::connection::{ConnectionMessage, Input, Output, ReplConf};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq)]
pub struct OutgoingHandshake {
    i: usize,
    requests: [Input; 4],
    responses: [Output; 4],
}

impl OutgoingHandshake {
    #[must_use]
    pub fn new() -> Self {
        Self {
            i: 0,
            requests: [
                Input::Ping,
                Input::ReplConf(ReplConf::ListingPort(1)),
                Input::ReplConf(ReplConf::Capa(String::new())),
                Input::Psync,
            ],
            responses: [
                Output::Pong,
                ReplConf::ListingPort(1).into(),
                ReplConf::Capa(String::new()).into(),
                Output::Psync,
            ],
        }
    }

    pub fn next(&mut self) {
        self.i += 1;
    }

    #[must_use]
    pub fn get_message(&self) -> Input {
        self.requests[self.i].clone()
    }

    pub fn expected_message(&self) -> Output {
        self.responses[self.i].clone()
    }

    pub fn handle_response(&mut self, message: ConnectionMessage) -> anyhow::Result<()> {
        let response = message.into_output().unwrap();
        let expected = self.expected_message();
        assert_eq!(response, expected);
        self.next();
        Ok(())
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.i == self.requests.len()
    }

    pub fn get_all_messages(mut self) -> Vec<Input> {
        self.requests[self.i..].to_vec()
    }

    pub fn try_advance(&mut self, response: Output) -> anyhow::Result<Input> {
        bail!("err")
    }
}
