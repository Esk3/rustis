use crate::connection::{Input, Output};

#[cfg(test)]
mod tests;

pub struct ClientHandler;

impl ClientHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_request(&mut self, request: Input) -> Output {
        match request {
            Input::Ping => Output::Pong,
            Input::Get(_) => todo!(),
            Input::Set {
                key,
                value,
                expiry,
                get,
            } => todo!(),
            Input::Multi => todo!(),
            Input::CommitMulti => todo!(),
            Input::ReplConf(_) => todo!(),
            Input::Psync => todo!(),
        }
    }
}
