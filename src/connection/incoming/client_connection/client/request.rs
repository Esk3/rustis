#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Request {
    pub request: crate::Request,
    pub timestamp: std::time::SystemTime,
}

impl Request {
    #[must_use]
    pub fn new(request: crate::Request, timestamp: std::time::SystemTime) -> Self {
        Self { request, timestamp }
    }
    #[must_use]
    pub fn now(request: crate::Request) -> Self {
        Self::new(request, std::time::SystemTime::now())
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn epoch(request: crate::Request) -> Self {
        Self::new(request, std::time::SystemTime::UNIX_EPOCH)
    }

    pub fn into_content(self) -> Result<Vec<String>, Self> {
        match self.request {
            crate::Request::Standard(s) => Ok(s.args),
        }
    }
}

impl std::ops::Deref for Request {
    type Target = crate::Request;

    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl From<Request> for crate::Request {
    fn from(value: Request) -> Self {
        value.request
    }
}
