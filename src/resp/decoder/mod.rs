use std::collections::VecDeque;

pub struct Decoder<R> {
    r: std::io::BufReader<R>,
    tokens: VecDeque<()>,
}

impl<R> Decoder<R>
where
    R: std::io::Read,
{
    pub fn new(r: R) -> Self {
        Self {
            r: std::io::BufReader::new(r),
            tokens: VecDeque::new(),
        }
    }
    pub fn decode_token(&mut self) {}
    pub fn decode_array(&mut self) {}
}
