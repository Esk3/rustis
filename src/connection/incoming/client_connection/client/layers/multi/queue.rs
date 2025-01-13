use crate::connection::incoming::client_connection::client;

pub struct Queue(Option<Vec<client::Request>>);

impl Queue {
    #[must_use]
    pub fn new() -> Self {
        Self(None)
    }
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.0.is_some()
    }
    pub fn store(&mut self, request: client::Request) -> StoreResult {
        match &mut self.0 {
            None if request.value.first().unwrap().eq_ignore_ascii_case("Multi") => {
                self.0 = Some(Vec::new());
                StoreResult::Ok
            }
            Some(_) if request.value.first().unwrap().eq_ignore_ascii_case("Multi") => {
                StoreResult::InvalidStore(request)
            }
            None => StoreResult::InvalidStore(request),
            Some(_) if request.value.first().unwrap().eq_ignore_ascii_case("Exec") => {
                StoreResult::QueueFinished(self.0.take().unwrap())
            }
            Some(list) => {
                list.push(request);
                StoreResult::Ok
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StoreResult {
    Ok,
    InvalidStore(client::Request),
    QueueFinished(Vec<client::Request>),
}
