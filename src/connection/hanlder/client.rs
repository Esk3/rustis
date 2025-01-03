use std::fmt::Debug;

use tracing::instrument;

use crate::{
    connection::{Input, Output},
    event::EventEmitter,
    repository::Repository,
};

#[derive(Debug)]
pub struct ClientState {
    repo: Repository,
    event: EventEmitter,
    queue: Option<Vec<Input>>,
}

impl ClientState {
    pub fn new(event: EventEmitter, repo: Repository) -> Self {
        Self {
            repo,
            event,
            queue: None,
        }
    }

    pub fn event(&self) -> &EventEmitter {
        &self.event
    }
}

#[instrument]
pub fn handle_client_request(
    request: Input,
    state: &mut ClientState,
) -> anyhow::Result<ClientResult> {
    tracing::debug!("handling request: {request:?}");
    match replication_layer(&request) {
        ReplicationResult::Replicate => return Ok(ClientResult::BecomeFollower),
        ReplicationResult::Continue => (),
    }
    let response = transaction_layer(request, state)?;
    Ok(ClientResult::SendOutput(response))
}

fn replication_layer(request: &Input) -> ReplicationResult {
    match request {
        Input::ReplConf(_) | Input::Psync => ReplicationResult::Replicate,
        _ => ReplicationResult::Continue,
        Input::Ping => todo!(),
        Input::Get(_) => todo!(),
        Input::Set {
            key,
            value,
            expiry,
            get,
        } => todo!(),
        Input::Multi => todo!(),
        Input::CommitMulti => todo!(),
    }
}

fn transaction_layer(request: Input, state: &mut ClientState) -> anyhow::Result<Output> {
    if let Some(ref mut queue) = state.queue {
        if let Input::CommitMulti = request {
            let queue = state.queue.take().unwrap();
            let responses = queue
                .into_iter()
                .map(|req| handler(req, state).unwrap())
                .collect();
            Ok(Output::Array(responses))
        } else {
            queue.push(request);
            Ok(Output::Queued)
        }
    } else if let Input::Multi = request {
        state.queue = Some(Vec::new());
        Ok(Output::Multi)
    } else {
        handler(request, state)
    }
}

fn handler(request: Input, state: &mut ClientState) -> anyhow::Result<Output> {
    let response = match request {
        Input::Ping => Output::Pong,
        Input::Get(key) => {
            let value = state.repo.get(&key)?;
            Output::Get(value)
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
            Output::Set
        }
        Input::CommitMulti | Input::Multi | Input::ReplConf(_) | Input::Psync => unreachable!(),
    };
    tracing::debug!("response: {response:?}");
    Ok(response)
}

#[derive(Debug)]
pub enum ReplicationResult {
    Replicate,
    Continue,
}

#[derive(Debug)]
pub enum ClientResult {
    None,
    SendOutput(Output),
    BecomeFollower,
}

#[cfg(test)]
mod tests {
    use crate::{
        connection::{Input, Output, ReplConf},
        event::{EventEmitter, Kind},
        repository::Repository,
    };

    use super::{handle_client_request, ClientResult, ClientState};

    #[test]
    fn ping() {
        let event = EventEmitter::new();
        let repo = Repository::new();
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
        let event = EventEmitter::new();
        let repo = Repository::new();
        let mut client = ClientState::new(event, repo);
        let res = handle_client_request(Input::Get("abc".into()), &mut client).unwrap();
        let ClientResult::SendOutput(res) = res else {
            panic!();
        };
        assert_eq!(res, Output::Get(None));
    }

    #[test]
    fn set_and_get_value() {
        let event = EventEmitter::new();
        let repo = Repository::new();
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
        let event = EventEmitter::new();
        let repo = Repository::new();
        let mut client = ClientState::new(event, repo);
        let ClientResult::BecomeFollower =
            handle_client_request(Input::ReplConf(ReplConf::ListingPort(1)), &mut client).unwrap()
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
        let repo = Repository::new();
        let (key, value) = ("abc", "xyz");
        //let event = EventEmitter::new(vec![Kind::Set {
        //    key: key.into(),
        //    value: value.into(),
        //    expiry: (),
        //}]);
        let event = EventEmitter::new();

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

    #[test]
    fn start_multi() {
        let mut state = ClientState::new(EventEmitter::new(), Repository::new());
        let ClientResult::SendOutput(Output::Multi) =
            handle_client_request(Input::Multi, &mut state).unwrap()
        else {
            panic!();
        };
    }

    #[test]
    fn queue_multi() {
        let mut state = ClientState::new(EventEmitter::new(), Repository::new());
        let ClientResult::SendOutput(Output::Multi) =
            handle_client_request(Input::Multi, &mut state).unwrap()
        else {
            panic!();
        };
        let (key, value) = ("abc", "xyz");
        let ClientResult::SendOutput(Output::Queued) = handle_client_request(
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

    #[test]
    fn commit_empty_multi() {
        let mut state = ClientState::new(EventEmitter::new(), Repository::new());
        let ClientResult::SendOutput(Output::Multi) =
            handle_client_request(Input::Multi, &mut state).unwrap()
        else {
            panic!();
        };
        let ClientResult::SendOutput(Output::Array(arr)) =
            handle_client_request(Input::CommitMulti, &mut state).unwrap()
        else {
            panic!();
        };
        assert!(arr.is_empty());
    }

    #[test]
    #[ignore = "reason"]
    fn commit_whilte_not_in_multi_errors() {
        todo!()
    }

    #[test]
    fn commit_multi() {
        let repo = Repository::new();
        let mut state = ClientState::new(EventEmitter::new(), repo.clone());
        let ClientResult::SendOutput(Output::Multi) =
            handle_client_request(Input::Multi, &mut state).unwrap()
        else {
            panic!();
        };
        let (key, value) = ("abc", "xyz");
        let ClientResult::SendOutput(Output::Queued) = handle_client_request(
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
        let ClientResult::SendOutput(Output::Array(arr)) =
            handle_client_request(Input::CommitMulti, &mut state).unwrap()
        else {
            panic!();
        };
        assert_eq!(arr, [Output::Set]);
        let v = repo.get(key).unwrap().unwrap();
        assert_eq!(v, value);
    }

    #[test]
    fn repo_is_not_updated_until_commit() {
        let repo = Repository::new();
        let mut state = ClientState::new(EventEmitter::new(), repo.clone());
        let ClientResult::SendOutput(Output::Multi) =
            handle_client_request(Input::Multi, &mut state).unwrap()
        else {
            panic!();
        };
        let (key, value) = ("abc", "xyz");
        let ClientResult::SendOutput(Output::Queued) = handle_client_request(
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
        assert!(repo.get(key).unwrap().is_none());
        let ClientResult::SendOutput(Output::Array(arr)) =
            handle_client_request(Input::CommitMulti, &mut state).unwrap()
        else {
            panic!();
        };
        assert_eq!(arr, [Output::Set]);
        let v = repo.get(key).unwrap().unwrap();
        assert_eq!(v, value);
    }

    #[test]
    fn abort_multi() {}
}
