use tracing::instrument;

use crate::{
    event::EventProducer,
    io::{Input, Output},
    repository::Repository,
};

pub struct LeaderState<R, E> {
    repo: R,
    event_emitter: E,
}

impl<R, E> LeaderState<R, E>
where
    R: Repository,
    E: EventProducer,
{
    pub fn new(event_emitter: E, repo: R) -> Self {
        Self {
            repo,
            event_emitter,
        }
    }
}

#[instrument(skip(state))]
pub fn handle_message_from_leader<R, E>(
    message: Input,
    state: &mut LeaderState<R, E>,
) -> anyhow::Result<Response>
where
    R: Repository,
    E: EventProducer,
{
    let response = match message {
        Input::Ping => Response::None,
        Input::Get(_) => todo!(),
        Input::Set {
            key,
            value,
            expiry,
            get,
        } => {
            state.repo.set(key.clone(), value.clone(), expiry).unwrap();
            state.event_emitter.emmit(crate::event::Kind::Set {
                key,
                value,
                expiry: (),
            });
            Response::None
        }
        Input::Multi => todo!(),
        Input::CommitMulti => todo!(),
        Input::ReplConf(_) => todo!(),
        Input::Psync => todo!(),
    };
    Ok(response)
}

#[derive(Debug)]
pub enum Response {
    None,
    SendOutput(Output),
}

#[cfg(test)]
mod tests {
    use crate::{
        connection::hanlder::leader::{handle_message_from_leader, LeaderState, Response},
        event::{
            tests::{MockEventProducer, MockEventProducerSink},
            Kind,
        },
        repository::{LockingMemoryRepository, Repository},
    };

    #[test]
    fn sets_silently() {
        let repo = LockingMemoryRepository::new();
        let mut state = LeaderState::new(MockEventProducerSink, repo.clone());
        let (key, value) = ("abc", "xyz");
        let Response::None = handle_message_from_leader(
            crate::io::Input::Set {
                key: key.into(),
                value: value.into(),
                expiry: None,
                get: false,
            },
            &mut state,
        )
        .unwrap() else {
            panic!()
        };
        assert_eq!(repo.get(key).unwrap().unwrap(), value);
    }

    #[test]
    fn emmits_event_on_set() {
        let (key, value) = ("abc", "xyz");
        let event = MockEventProducer::new([Kind::Set {
            key: key.into(),
            value: value.into(),
            expiry: (),
        }]);
        let mut state = LeaderState::new(event, LockingMemoryRepository::new());
        let Response::None = handle_message_from_leader(
            crate::io::Input::Set {
                key: key.into(),
                value: value.into(),
                expiry: None,
                get: false,
            },
            &mut state,
        )
        .unwrap() else {
            panic!()
        };
    }

    #[test]
    #[ignore = "todo"]
    fn returns_bytes_processed_after_no_messages() {
        todo!()
    }

    #[test]
    #[ignore = "todo"]
    fn returns_bytes_processed_after_messages() {
        todo!()
    }
}
