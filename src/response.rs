use std::fmt;

use crate::prelude::*;

#[derive(Debug)]
pub struct Response {
    pub protocol: Protocol,
    pub code: StatusCode,
    pub string: String,
}

impl Response {
    pub fn new(code: StatusCode, content: &str) -> Self {
        let protocol = Protocol::Http1;
        let length = content.len();
        let string = format!("{protocol} {code}\r\nContent-Length: {length}\r\n\r\n{content}");

        Self {
            protocol,
            code,
            string,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Response: {} {}", self.protocol, self.code)
    }
}
