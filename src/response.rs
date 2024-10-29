use flate2::{write::GzEncoder, Compression};
use std::{collections::HashMap, fmt, io::Write};

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
    pub fn encode(&mut self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        output.extend(format!("{} {}\r\n", self.protocol, self.code).as_bytes());
        output.extend(
            self.headers
                .iter()
                .flat_map(|(k, v)| format!("{k}: {v}\r\n").into_bytes()),
        );

        if let Some(content) = &self.content {
            let mut buffer = Vec::new();
            let unencoded = content.body.as_bytes();

            let body = if self.encoding == Some(Encoding::Gzip) {
                let gzip_ok = {
                    let mut encoder = GzEncoder::new(&mut buffer, Compression::default());

                    encoder
                        .write_all(unencoded)
                        .and_then(|_| encoder.try_finish())
                        .is_ok()
                };

                if gzip_ok {
                    output.extend(format!("Content-Encoding: {}\r\n", Encoding::Gzip).as_bytes());
                    &buffer
                } else {
                    unencoded
                }
            } else {
                unencoded
            };

            output.extend(format!("Content-Type: {}\r\n", content.mime_type).as_bytes());
            output.extend(format!("Content-Length: {}\r\n", body.len()).as_bytes());
            output.extend(b"\r\n");
            output.extend(body);
        } else {
            output.extend(b"\r\n");
        }

        output
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
