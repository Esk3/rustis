pub mod node_worker;

pub trait NodeService {
    fn get(&self, key: String) -> Result<String, ()>;
}

#[cfg(test)]
pub struct DummyService;

#[cfg(test)]
impl NodeService for DummyService {
    fn get(&self, key: String) -> Result<String, ()> {
        Ok(format!("dummy response for key {key}"))
    }
}
