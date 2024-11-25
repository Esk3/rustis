pub mod node_worker;

pub trait NodeService {
    fn get(&self, key: String) -> Result<Option<String>, ()>;
    fn set(&self, key: String, value: String) -> Result<(), ()>;
    fn wait(&self, count: usize) -> Result<(), ()>;
}

#[cfg(test)]
pub mod tests {
    pub mod dymmy_service {
        use crate::node_service::NodeService;

        pub struct AlwaysOk;

        impl NodeService for AlwaysOk {
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

        pub struct NotFound;
        impl NodeService for NotFound {
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
    }
}
