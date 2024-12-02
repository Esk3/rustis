use node_worker::Kind;

pub mod node_worker;

pub trait ClientService {
    fn get(&self, key: String) -> Result<Option<String>, ()>;

    fn set(&self, _key: String, _value: String) -> Result<(), ()>;

    fn wait(&self, _count: usize) -> Result<(), ()>;
}

pub trait FollowerService {
    fn get_event_from_node(&self) -> Kind;
    fn get_follower_byte_offset(&self) -> Kind;
}

pub trait LeaderService {
    fn get_event_from_leader(&self) -> Kind;
}

#[cfg(test)]
pub mod tests {
    pub mod dummy_service {
        use crate::node_service::{
            node_worker::Kind, ClientService, FollowerService, LeaderService,
        };

        pub struct AlwaysOk;

        impl ClientService for AlwaysOk {
            fn get(&self, key: String) -> Result<Option<String>, ()> {
                Ok(format!("dummy response for key {key}").into())
            }

            fn set(&self, _key: String, _value: String) -> Result<(), ()> {
                Ok(())
            }

            fn wait(&self, _count: usize) -> Result<(), ()> {
                Ok(())
            }
        }

        impl FollowerService for AlwaysOk {
            fn get_event_from_node(&self) -> Kind {
                Kind::ReplicateSet {
                    key: "dummy".to_string(),
                    value: "test".to_string(),
                    expiry: None,
                }
            }

            fn get_follower_byte_offset(&self) -> Kind {
                std::todo!()
            }
        }
        impl LeaderService for AlwaysOk {
            fn get_event_from_leader(&self) -> Kind {
                Kind::ReplicateSet {
                    key: "dummy".to_string(),
                    value: "test".to_string(),
                    expiry: None,
                }
            }
        }

        pub struct NotFound;
        impl ClientService for NotFound {
            fn get(&self, _key: String) -> Result<Option<String>, ()> {
                Ok(None)
            }

            fn set(&self, _key: String, _value: String) -> Result<(), ()> {
                Ok(())
            }

            fn wait(&self, _count: usize) -> Result<(), ()> {
                Ok(())
            }
        }

        #[test]
        fn test() {
            let s = AlwaysOk;
            let event = s.get_event_from_node();
            let expected = Kind::ReplicateSet {
                key: "dummy".to_string(),
                value: "test".to_string(),
                expiry: None,
            };
            assert_eq!(event, expected);
            let event = s.get_event_from_leader();
            assert_eq!(event, expected);
        }
    }
}
