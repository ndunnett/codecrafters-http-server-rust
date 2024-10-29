use std::fmt;

pub use crate::{error::Error, request::Request, response::Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Protocol {
    Http1,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http1 => write!(f, "HTTP/1.1"),
        }
    }
}

impl TryFrom<&str> for Protocol {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "HTTP/1.1" => Ok(Self::Http1),
            _ => Err(Error::Generic("Failed to parse HTTP version.".into())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalError = 500,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Ok => "OK",
            Self::Created => "Created",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::MethodNotAllowed => "Method Not Allowed",
            Self::InternalError => "Internal Error",
        };

        write!(f, "{} {msg}", self.clone() as i32)
    }
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
        }
    }
}

impl TryFrom<&str> for Method {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            _ => Err(Error::Generic("Failed to parse request method.".into())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MimeType {
    PlainText,
    Html,
    OctetStream,
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlainText => write!(f, "text/plain"),
            Self::Html => write!(f, "text/html"),
            Self::OctetStream => write!(f, "application/octet-stream"),
        }
    }
}

impl TryFrom<&str> for MimeType {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "text/plain" => Ok(Self::PlainText),
            "text/html" => Ok(Self::Html),
            "application/octet-stream" => Ok(Self::OctetStream),
            _ => Err(Error::Generic("Failed to parse mime type.".into())),
        }
    }
}

impl TryFrom<&String> for MimeType {
    type Error = Error;

    fn try_from(s: &String) -> Result<Self> {
        Self::try_from(s.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Content {
    pub mime_type: MimeType,
    pub body: String,
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Content({}, {} B)", self.mime_type, self.body.len())
    }
}

impl From<&Content> for String {
    fn from(content: &Content) -> Self {
        format!(
            "Content-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            content.mime_type,
            content.body.len(),
            content.body
        )
    }
}

impl Content {
    pub fn new(mime_type: MimeType, body: &str) -> Self {
        Self {
            mime_type,
            body: body.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Encoding {
    Gzip,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gzip => write!(f, "gzip"),
        }
    }
}

impl TryFrom<&str> for Encoding {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "gzip" => Ok(Self::Gzip),
            _ => Err(Error::Generic("Failed to parse encoding method.".into())),
        }
    }
}

impl TryFrom<&String> for Encoding {
    type Error = Error;

    fn try_from(s: &String) -> Result<Self> {
        Self::try_from(s.as_str())
    }
}
