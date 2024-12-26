use std::fmt::Debug;

use hanlder::{
    client::{handle_client_request, ClientResult, ClientState},
    follower::{handle_follower_event, FollowerState},
};
use thiserror::Error;
use tracing::instrument;

use crate::{
    event::{EventProducer, EventSubscriber},
    io::{Io, IoError, NetworkMessage},
    repository::Repository,
    resp::parser::{Encode, Parse},
};

pub mod hanlder;

#[instrument(skip(repo))]
pub fn connection_handler<P, En, I, E, R>(
    mut io: I,
    event: E,
    repo: R,
) -> Result<(), ConnectionError>
where
    P: Parse,
    En: Encode,
    I: Io + Debug,
    E: EventProducer + Debug,
    R: Repository + Debug,
    <E as EventProducer>::Subscriber: std::fmt::Debug,
{
    let mut state = ClientState::new(event, repo);

    loop {
        let value = io.read_value()?;
        tracing::debug!("got value {value:?}");
        let message = P::parse(value).unwrap();
        let NetworkMessage::Input(input) = message else {
            todo!()
        };
        let response = handle_client_request(input, &mut state).unwrap();
        match response {
            ClientResult::None => (),
            ClientResult::SendOutput(output) => {
                let value = En::encode(NetworkMessage::Output(output)).unwrap();
                io.write_value(value).unwrap();
            }
            ClientResult::BecomeFollower => break,
        }
    }

    let mut state = FollowerState::new(state.event.subscribe());
    loop {
        let event = state.subscriber.recive();
        let result = handle_follower_event(event, &mut state).unwrap();
        match result {
            hanlder::follower::FollowerResult::None => (),
            hanlder::follower::FollowerResult::SendOutput(output) => {
                let value = En::encode(NetworkMessage::Output(output)).unwrap();
                io.write_value(value).unwrap();
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("io error {0}")]
    IoError(#[from] IoError),
}

#[instrument]
pub fn leader_connection_handler() {}

#[cfg(test)]
mod tests {
    use crate::{
        connection::ConnectionError,
        event::tests::MockEventProducerSink,
        io::tests::MockIo,
        repository::LockingMemoryRepository,
        resp::{
            parser::{RespEncoder, RespParser},
            Value,
        },
    };

    use super::connection_handler;

    #[test_log::test]
    fn ping_client() {
        let io = MockIo::new(
            [
                Value::Array(vec![Value::BulkString("PING".into())]),
                Value::SimpleString("end".into()),
            ],
            [Value::SimpleString("PONG".into())],
        );
        let ConnectionError::IoError(crate::io::IoError::EndOfInput) =
            connection_handler::<RespParser, RespEncoder, _, _, _>(
                io,
                MockEventProducerSink,
                LockingMemoryRepository::new(),
            )
            .unwrap_err()
        else {
            panic!()
        };
    }
}
