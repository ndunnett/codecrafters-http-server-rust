use std::{
    collections::HashMap,
    fmt,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use crate::prelude::*;

fn read_section(reader: &mut BufReader<&mut TcpStream>) -> Result<String> {
    let mut buffer = Vec::new();

    loop {
        match reader.read_until(b'\n', &mut buffer) {
            Ok(0) => break,
            Ok(_) if buffer.ends_with(b"\r\n") => break,
            Err(e) => return Err(e.into()),
            _ => {
                if buffer.len() > 1024 {
                    return Err("Request exceeds 1024 bytes.".into());
                }
            }
        }
    }

    Ok(String::from_utf8(buffer)?.trim().to_string())
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub protocol: Protocol,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn parse(stream: &mut TcpStream) -> Result<Self> {
        let mut reader = BufReader::new(stream);

        let (method, uri, protocol) = {
            let status_line = read_section(&mut reader)?;

            match status_line.split(" ").collect::<Vec<_>>()[..] {
                [method, uri, protocol] => (
                    Method::try_from(method)?,
                    String::from(uri),
                    Protocol::try_from(protocol)?,
                ),
                _ => {
                    return Err("Failed to parse request line.".into());
                }
            }
        };

        let headers = {
            let section = read_section(&mut reader)?;
            let mut headers = HashMap::new();

            for line in section.lines() {
                if let Some((k, v)) = line.split_once(": ") {
                    headers.insert(k.into(), v.into());
                } else {
                    break;
                }
            }

            headers
        };

        let body = read_section(&mut reader)?;

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
