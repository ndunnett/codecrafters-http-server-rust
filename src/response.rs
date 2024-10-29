use std::{collections::HashMap, fmt};

use crate::prelude::*;

#[derive(Debug)]
pub struct Response {
    pub protocol: Protocol,
    pub code: StatusCode,
    pub content: Option<Content>,
    pub headers: HashMap<String, String>,
    pub encoding: Option<Encoding>,
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut lines = Vec::with_capacity(self.headers.len() + 2);
        lines.push(format!("{} {}", self.protocol, self.code));

        for header in self.headers.iter().map(|(k, v)| format!("{k}: {v}")) {
            lines.push(header);
        }

        if let Some(content) = &self.content {
            lines.push(content.into());
        } else {
            lines.push("\r\n".into());
        }

        lines.join("\r\n").into_bytes()
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
