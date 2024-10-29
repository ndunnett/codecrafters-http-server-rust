use core::num;
use std::{error, fmt, io, string};

pub enum Error {
    Generic(String),
    IO(io::Error),
    FromUtf8(string::FromUtf8Error),
    ParseInt(num::ParseIntError),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Generic(_) => None,
            Self::IO(e) => Some(e),
            Self::FromUtf8(e) => Some(e),
            Self::ParseInt(e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(s) => write!(f, "Error: {s}"),
            Self::IO(e) => write!(f, "IO Error: {e}"),
            Self::FromUtf8(e) => write!(f, "UTF-8 Error: {e}"),
            Self::ParseInt(e) => write!(f, "Parsing Error: {e}"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base = match self {
            Self::Generic(s) => s.clone(),
            Self::IO(e) => format!("{e:#?}"),
            Self::FromUtf8(e) => format!("{e:#?}"),
            Self::ParseInt(e) => format!("{e:#?}"),
        };

        let s = base
            .split('\n')
            .map(|l| format!("|   {l}"))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "| Error:\n{s}")
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Generic(s.into())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Self::FromUtf8(e)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}
