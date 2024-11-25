use crate::connection::ConnectionInputOutput;

pub struct Api<S> {
    stream: S,
}

impl<S> Api<S> where S: std::io::Read + std::io::Write {}

impl<S> ConnectionInputOutput for Api<S>
where
    S: std::io::Read + std::io::Write,
{
    fn get_request(&mut self) -> Result<crate::connection::request::Request, ()> {
        todo!()
    }

    fn send_response(&mut self, response: crate::connection::response::Response) -> Result<(), ()> {
        todo!()
    }
}
