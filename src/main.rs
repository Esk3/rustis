use rustis::{
    connection::{handle_client_request, ClientState},
    event::LockEventProducer,
    repository::MemoryRepository,
};

fn main() {
    tracing_subscriber::fmt::init();
    let res = handle_client_request(
        rustis::io::Input::Ping,
        &mut ClientState::new(LockEventProducer::new(), MemoryRepository::new()),
    );
    panic!();
}
