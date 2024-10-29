use std::{
    collections::HashMap,
    fmt,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use crate::prelude::*;

const MAX_REQUEST_SIZE: usize = 4096;

fn read_until(reader: &mut BufReader<&mut TcpStream>, bytes: &[u8]) -> Result<String> {
    let mut buffer = Vec::new();

    loop {
        match reader.read_until(bytes[bytes.len() - 1], &mut buffer) {
            Ok(0) => break,
            Ok(_) if buffer.ends_with(bytes) => break,
            Err(e) => return Err(e.into()),
            _ if buffer.len() > MAX_REQUEST_SIZE => {
                return Err(Error::Generic(format!(
                    "Request exceeds {MAX_REQUEST_SIZE} bytes."
                )))
            }
            _ => {}
        }
    }

    Ok(String::from_utf8(buffer)?)
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub protocol: Protocol,
    pub headers: HashMap<String, String>,
    pub content: Option<Content>,
}

impl Request {
    pub fn parse(stream: &mut TcpStream) -> Result<Self> {
        let mut reader = BufReader::new(stream);

        let header = read_until(&mut reader, b"\r\n\r\n")?;
        let mut lines = header.trim().lines();

        let (method, uri, protocol) = {
            let status_line = lines
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

            for line in lines {
                if let Some((k, v)) = line.split_once(": ") {
                    headers.insert(String::from(k), String::from(v));
                } else {
                    break;
                }
            }

            headers
        };

        let content = {
            match (headers.get("Content-Type"), headers.get("Content-Length")) {
                (Some(mime_type), Some(length)) => {
                    let length = length.parse::<usize>()?;
                    let mut buffer = [0; MAX_REQUEST_SIZE];
                    let mut bytes = 0;

                    while bytes < length {
                        match reader.read(&mut buffer)? {
                            0 => break,
                            n => bytes += n,
                        }
                    }

                    let body = String::from_utf8(buffer[0..bytes].to_vec())?;
                    Some(Content::new(mime_type.try_into()?, &body))
                }
                _ => None,
            }
        };

        Ok(Self {
            method,
            uri,
            protocol,
            headers,
            content,
        })
    }

    pub fn response(&self, code: StatusCode, content: Option<Content>) -> Result<Response> {
        let mut headers = HashMap::new();
        let mut encoding = None;

        if let Some(schemes) = self.headers.get("Accept-Encoding") {
            let mut schemes = schemes
                .split(", ")
                .filter_map(|s| Encoding::try_from(s).ok())
                .collect::<Vec<_>>();

            if !schemes.is_empty() {
                schemes.sort();

                if let Some(scheme) = schemes.first() {
                    headers.insert("Content-Encoding".to_string(), scheme.to_string());
                    encoding = Some(scheme.clone());
                }
            }
        }

        Ok(Response {
            protocol: self.protocol.clone(),
            code,
            content,
            headers,
            encoding,
        })
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Request: {} {} {}", self.method, self.uri, self.protocol)?;

        if let Some(content) = &self.content {
            write!(f, " -> {content}")?;
        }

        Ok(())
    }
}
