use std::fmt::Debug;

use tracing::instrument;

use crate::{
    event::{EventProducer, EventSubscriber, Kind},
    io::{Input, Output},
    repository::Repository,
};

#[derive(Debug)]
pub struct ClientState<R, E> {
    repo: R,
    event: E,
}

impl<R, E> ClientState<R, E>
where
    R: Repository,
    E: EventProducer,
{
    pub fn new(event: E, repo: R) -> Self {
        Self { repo, event }
    }
}

#[instrument]
pub fn handle_client_request<R, E>(
    request: Input,
    state: &mut ClientState<R, E>,
) -> anyhow::Result<ClientResult>
where
    R: Repository + Debug,
    E: EventProducer + Debug,
{
    tracing::trace!("handling request: {request:?}");
    let response = match request {
        Input::Ping => ClientResult::SendOutput(Output::Pong),
        Input::Get(key) => {
            let value = state.repo.get(&key)?;
            ClientResult::SendOutput(Output::Get(value))
        }
        Input::Set {
            key,
            value,
            expiry,
            get,
        } => {
            state.repo.set(key.clone(), value.clone(), expiry)?;
            state.event.emmit(crate::event::Kind::Set {
                key,
                value,
                expiry: (),
            });
            assert!(!get, "todo return old value");
            ClientResult::SendOutput(Output::Set)
        }
        Input::ReplConf | Input::Psync => ClientResult::BecomeFollower,
    };
    tracing::trace!("response: {response:?}");
    Ok(response)
}

#[derive(Debug)]
pub enum ClientResult {
    None,
    SendOutput(Output),
    BecomeFollower,
}

pub struct Leader {}

pub struct Follower<E> {
    event_reciver: E,
}

impl<E> Follower<E>
where
    E: EventSubscriber,
{
    pub fn new(event_reciver: E) -> Self {
        todo!()
    }
}

#[instrument]
pub fn handle_follower_event(event: Kind) {
    tracing::trace!("handling event {event:?}");
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{
        event::{
            tests::{MockEventProducer, MockEventProducerSink},
            Kind,
        },
        io::{Input, Output},
        repository::MemoryRepository,
    };

    use super::{handle_client_request, ClientResult, ClientState};

    #[test]
    fn ping() {
        let event = MockEventProducerSink;
        let repo = MemoryRepository::new();
        let mut client = ClientState::new(event, repo);
        let input = Input::Ping;
        let res = handle_client_request(input, &mut client).unwrap();
        let ClientResult::SendOutput(res) = res else {
            panic!();
        };
        assert_eq!(res, Output::Pong);
    }

    #[test]
    fn get_unset() {
        let event = MockEventProducerSink;
        let repo = MemoryRepository::new();
        let mut client = ClientState::new(event, repo);
        let res = handle_client_request(Input::Get("abc".into()), &mut client).unwrap();
        let ClientResult::SendOutput(res) = res else {
            panic!();
        };
        assert_eq!(res, Output::Get(None));
    }

    #[test]
    fn set_and_get_value() {
        let event = MockEventProducerSink;
        let repo = MemoryRepository::new();
        let mut client = ClientState::new(event, repo);
        let (key, value) = ("abc", "xyz");
        let res = handle_client_request(
            Input::Set {
                key: key.into(),
                value: value.into(),
                expiry: None,
                get: false,
            },
            &mut client,
        )
        .unwrap();
        let ClientResult::SendOutput(res) = res else {
            panic!();
        };
        assert_eq!(res, Output::Set);
        let res = handle_client_request(Input::Get(key.into()), &mut client).unwrap();
        let ClientResult::SendOutput(res) = res else {
            panic!()
        };
        assert_eq!(res, Output::Get(Some(value.into())));
    }

    #[test]
    fn repl_conf_returns_into_follower() {
        let event = MockEventProducerSink;
        let repo = MemoryRepository::new();
        let mut client = ClientState::new(event, repo);
        let ClientResult::BecomeFollower =
            handle_client_request(Input::ReplConf, &mut client).unwrap()
        else {
            panic!();
        };
        let ClientResult::BecomeFollower =
            handle_client_request(Input::Psync, &mut client).unwrap()
        else {
            panic!();
        };
    }

    #[test]
    fn handler_send_event_on_set() {
        let repo = MemoryRepository::new();
        let (key, value) = ("abc", "xyz");
        let event = MockEventProducer::new(vec![Kind::Set {
            key: key.into(),
            value: value.into(),
            expiry: (),
        }]);

        let mut state = ClientState::new(event, repo);
        let ClientResult::SendOutput(_output) = handle_client_request(
            Input::Set {
                key: key.into(),
                value: value.into(),
                expiry: None,
                get: false,
            },
            &mut state,
        )
        .unwrap() else {
            panic!();
        };
    }
}
