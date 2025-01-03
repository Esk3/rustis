use anyhow::bail;

use crate::connection::{Input, Output, ReplConf};

#[cfg(test)]
mod tests;

pub struct IncomingHandshake {
    input: Vec<Input>,
    output: Vec<Output>,
}
impl IncomingHandshake {
    pub fn new() -> Self {
        Self {
            input: [
                Input::Ping,
                ReplConf::ListingPort(1).into(),
                ReplConf::Capa(String::new()).into(),
                Input::Psync,
            ]
            .to_vec(),
            output: [
                Output::Pong,
                ReplConf::ListingPort(1).into(),
                ReplConf::Capa(String::new()).into(),
                Output::Psync,
            ]
            .to_vec(),
        }
    }

    pub fn get_all_messages(&self) -> Vec<Input> {
        self.input.clone()
    }

    pub fn get_all_responses(&self) -> Vec<Output> {
        self.output.clone()
    }

    pub fn handle_message_recived(&mut self, response: Input) -> anyhow::Result<()> {
        if response != self.input[0] {
            bail!("expected : {:?}", self.input[0]);
        }
        self.advance();
        Ok(())
    }

    pub fn get_message(&self) -> Option<Output> {
        self.output.first().cloned()
    }

    fn advance(&mut self) {
        self.input.remove(0);
        self.output.remove(0);
    }
    pub fn is_finished(&self) -> bool {
        self.input.is_empty()
    }
}
