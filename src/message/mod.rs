use crate::resp;

pub mod request;

#[derive(Debug)]
pub struct Message<T> {
    content: T,
    length: usize,
}

impl<T> Message<T> {
    pub fn new(content: T, length: usize) -> Self {
        Self { content, length }
    }

    pub fn content(&self) -> &T {
        &self.content
    }
    pub fn into_content(self) -> T {
        self.content
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

pub enum Kind {
    Value(Message<resp::Value>),
    Request(Message<crate::Request>),
}
