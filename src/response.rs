use std::fmt;

use crate::prelude::*;

#[derive(Debug)]
pub struct Response {
    pub protocol: Protocol,
    pub code: StatusCode,
    pub content_type: MimeType,
    pub string: String,
}

impl Response {
    pub fn new(code: StatusCode, content: &str, content_type: MimeType) -> Self {
        let protocol = Protocol::Http1;
        let length = content.len();
        let mut lines = vec![format!("{protocol} {code}")];

        if length > 0 {
            lines.push(format!("Content-Type: {content_type}"));
            lines.push(format!("Content-Length: {length}"));
            lines.push(format!("\r\n{content}"));
        } else {
            lines.push("\r\n".into());
        }

        let string = lines.join("\r\n");

        Self {
            protocol,
            code,
            content_type,
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
