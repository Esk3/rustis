use tracing::instrument;

use crate::{
    connection::{Input, Output},
    event::EventEmitter,
    repository::Repository,
};

pub struct LeaderState {
    repo: Repository,
    event_emitter: EventEmitter,
}

impl LeaderState {
    pub fn new(event_emitter: EventEmitter, repo: Repository) -> Self {
        Self {
            repo,
            event_emitter,
        }
    }
}

#[instrument(skip(state))]
pub fn handle_message_from_leader(
    message: Input,
    state: &mut LeaderState,
) -> anyhow::Result<Response> {
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
        connection::{
            hanlder::leader::{handle_message_from_leader, LeaderState, Response},
            Input,
        },
        event::EventEmitter,
        repository::Repository,
    };

    #[test]
    fn sets_silently() {
        let repo = Repository::new();
        let mut state = LeaderState::new(EventEmitter::new(), repo.clone());
        let (key, value) = ("abc", "xyz");
        let Response::None = handle_message_from_leader(
            Input::Set {
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
        //let event = MockEventProducer::new([Kind::Set {
        //    key: key.into(),
        //    value: value.into(),
        //    expiry: (),
        //}]);
        let event = EventEmitter::new();
        let mut state = LeaderState::new(event, Repository::new());
        let Response::None = handle_message_from_leader(
            Input::Set {
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
