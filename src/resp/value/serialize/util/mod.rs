use crate::resp::value::identifier::Identifier;

#[cfg(test)]
mod tests;

pub trait ExtendLinefeed {
    fn extend_linefeed(&mut self);
}

impl ExtendLinefeed for Vec<u8> {
    fn extend_linefeed(&mut self) {
        self.extend(b"\r\n");
    }
}

pub trait ExtendIdentifier {
    fn extend_identifier(&mut self, identifier: &Identifier);
}

impl ExtendIdentifier for Vec<u8> {
    fn extend_identifier(&mut self, identifier: &Identifier) {
        self.push(identifier.as_byte());
    }
}

pub trait ExtendHeader {
    fn extend_header(&mut self, identifier: &Identifier, length: isize);
}

impl ExtendHeader for Vec<u8> {
    fn extend_header(&mut self, identifier: &Identifier, length: isize) {
        self.extend_identifier(identifier);
        self.extend(length.to_string().as_bytes());
        self.extend_linefeed();
    }
}
