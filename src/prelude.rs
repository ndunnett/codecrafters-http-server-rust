use std::fmt;

pub use crate::{error::Error, request::Request, response::Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
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
    NotFound = 404,
    InternalError = 500,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Ok => "OK",
            Self::NotFound => "Not Found",
            Self::InternalError => "Internal Error",
        };

        write!(f, "{} {msg}", self.clone() as i32)
    }
}

#[derive(Debug)]
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
    TextPlain,
    TextHtml,
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TextPlain => write!(f, "text/plain"),
            Self::TextHtml => write!(f, "text/html"),
        }
    }
}
