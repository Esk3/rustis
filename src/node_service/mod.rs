use node_worker::Kind;

pub mod node_worker;

pub trait ClientService {
    type F: FollowerService;
    fn get(&self, key: String) -> Result<Option<String>, ()>;

    fn set(&self, key: String, value: String) -> Result<(), ()>;

    fn wait(&self, count: usize) -> Result<(), ()>;
    fn into_follower(self) -> Self::F
    where
        Self: Sized;
}

pub trait FollowerService {
    fn get_event_from_node(&self) -> Kind;
    fn get_follower_byte_offset(&self) -> usize;
    fn wait_ack(&self);
}

pub trait LeaderService {
    fn get_event_from_leader(&self) -> Kind;
    fn set(&self, key: String, value: String) -> Result<(), ()>;
}

#[cfg(test)]
pub mod tests {
    pub mod dummy_service {
        use crate::node_service::{
            node_worker::Kind, ClientService, FollowerService, LeaderService,
        };

        pub struct AlwaysOk;

        impl ClientService for AlwaysOk {
            type F = Self;
            fn get(&self, key: String) -> Result<Option<String>, ()> {
                Ok(format!("dummy response for key {key}").into())
            }

            fn set(&self, _key: String, _value: String) -> Result<(), ()> {
                Ok(())
            }

            fn wait(&self, _count: usize) -> Result<(), ()> {
                Ok(())
            }

            fn into_follower(self) -> Self::F
            where
                Self: Sized,
            {
                Self
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

            fn get_follower_byte_offset(&self) -> usize {
                std::todo!()
            }

            fn wait_ack(&self) {
                todo!()
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

            fn set(&self, key: String, value: String) -> Result<(), ()> {
                todo!()
            }
        }

        pub struct NotFound;
        impl ClientService for NotFound {
            type F = Self;
            fn get(&self, _key: String) -> Result<Option<String>, ()> {
                Ok(None)
            }

            fn set(&self, _key: String, _value: String) -> Result<(), ()> {
                Ok(())
            }

            fn wait(&self, _count: usize) -> Result<(), ()> {
                Ok(())
            }

            fn into_follower(self) -> Self::F
            where
                Self: Sized,
            {
                todo!()
            }
        }

        impl FollowerService for NotFound {
            fn get_event_from_node(&self) -> Kind {
                todo!()
            }

            fn get_follower_byte_offset(&self) -> usize {
                todo!()
            }

            fn wait_ack(&self) {
                todo!()
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
