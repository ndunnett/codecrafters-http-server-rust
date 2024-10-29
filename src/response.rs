use std::fmt;

use crate::prelude::*;

#[derive(Debug)]
pub struct Response {
    pub protocol: Protocol,
    pub code: StatusCode,
    pub content: Option<Content>,
}

impl Response {
    pub fn new(code: StatusCode, content: Option<Content>) -> Self {
        Self {
            protocol: Protocol::Http1,
            code,
            content,
        }
    }

    pub fn serve(content: Content) -> Result<Self> {
        Ok(Self::new(StatusCode::Ok, Some(content)))
    }

    pub fn empty(code: StatusCode) -> Result<Self> {
        Ok(Self::new(code, None))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let content = if let Some(content) = &self.content {
            String::from(content)
        } else {
            String::from("\r\n")
        };

        let string = format!("{} {}\r\n{}", self.protocol, self.code, content);
        string.into_bytes()
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Response: {} {}", self.protocol, self.code)?;

        if let Some(content) = &self.content {
            write!(f, " -> {content}")?;
        }

        Ok(())
    }
}
