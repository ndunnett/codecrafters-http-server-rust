use std::{collections::HashMap, fmt};

use crate::prelude::*;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub protocol: Protocol,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn parse(request: &str) -> Result<Self> {
        let mut header_lines = request.lines();

        let (method, uri, protocol) = {
            let status_line = header_lines
                .next()
                .ok_or(Error::Generic("Failed to parse status line.".into()))?;

            match status_line.split(" ").collect::<Vec<_>>()[..] {
                [method, uri, protocol] => (
                    Method::try_from(method)?,
                    String::from(uri),
                    Protocol::try_from(protocol)?,
                ),
                _ => {
                    return Err("Failed to parse status line.".into());
                }
            }
        };

        let headers = {
            let mut headers = HashMap::new();

            for line in header_lines {
                if let Some((k, v)) = line.split_once(": ") {
                    headers.insert(k.into(), v.into());
                } else {
                    break;
                }
            }

            headers
        };

        let body = String::from("");

        Ok(Self {
            method,
            uri,
            protocol,
            headers,
            body,
        })
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Request: {} {} {}", self.method, self.uri, self.protocol)
    }
}
