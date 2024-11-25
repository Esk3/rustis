use crate::node_service::ClientService;

pub struct Client<S> {
    service: S,
}

impl<S> Client<S>
where
    S: ClientService,
{
    #[must_use]
    pub fn new(service: S) -> Self {
        Self { service }
    }

    pub fn handle_echo(&self, echo: String) -> String {
        echo
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_get(&self, key: impl ToString) -> Result<Option<String>, ()> {
        match self.service.get(key.to_string()) {
            Ok(Some(value)) => Ok(Some(value)),
            Ok(None) => Ok(None),
            Err(()) => Err(()),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_set(&self, key: impl ToString, value: impl ToString) -> Result<(), ()> {
        match self.service.set(key.to_string(), value.to_string()) {
            Ok(()) => Ok(()),
            Err(()) => Err(()),
        }
    }

    pub fn handle_wait(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        node_service::{node_worker, tests::dymmy_service::AlwaysOk},
        repository::Repository,
    };

    use super::*;
    #[test]
    fn get_always_ok() {
        let c = Client::new(AlwaysOk);
        c.handle_get("abc").unwrap().unwrap();
    }

    #[test]
    fn empty_get_is_none() {
        let manager = node_worker::run(crate::node::Node, Repository::new());
        let c = Client::new(manager);
        assert!(c.handle_get("abc").unwrap().is_none());
    }

    #[test]
    fn get_some() {
        let manager = node_worker::run(crate::node::Node, Repository::new());
        let c = Client::new(manager);
        let key = "abc";
        let value = "xyz";
        c.handle_set(key.to_string(), value.to_string());
        assert_eq!(
            c.handle_get(key.to_string()).unwrap().unwrap(),
            value.to_string()
        );
    }
}
