#[cfg(test)]
mod tests;

pub struct ClientHandler;

impl ClientHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_request(&mut self, request: ()) -> () {
        ()
    }
}
