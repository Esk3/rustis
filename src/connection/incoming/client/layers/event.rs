use crate::{
    connection::incoming::client::{self, handler::Hanlder},
    event::{self, EventEmitter},
    resp::{Input, Output},
    Service,
};

pub struct EventLayer {
    emitter: EventEmitter,
    handler: Hanlder,
}

impl EventLayer {
    pub fn new(emitter: EventEmitter, handler: Hanlder) -> Self {
        Self { emitter, handler }
    }

    pub fn get_event(input: &Input) -> Option<event::Kind> {
        match input {
            Input::Ping | Input::Get(_) => None,
            Input::Set {
                key,
                value,
                expiry,
                get: _,
            } => Some(event::Kind::Set {
                key: key.to_string(),
                value: value.to_string(),
                expiry: *expiry,
            }),
            Input::Multi => todo!(),
            Input::CommitMulti => todo!(),
            Input::ReplConf(_) => todo!(),
            Input::Psync => todo!(),
            Input::XAdd => todo!(),
            Input::XRead => todo!(),
            Input::XRange => todo!(),
        }
    }
}

impl Service<client::Request> for EventLayer {
    type Response = Output;

    type Error = anyhow::Error;

    fn call(&mut self, request: client::Request) -> Result<Self::Response, Self::Error> {
        let get_event = Self::get_event(&request.input);

        let result = self.handler.call(request);

        if let Some(event) = get_event.filter(|_| result.is_ok()) {
            self.emitter.emmit(event);
        }
        result
    }
}
