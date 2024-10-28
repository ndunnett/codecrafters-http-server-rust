use std::{error, fmt, io, string};

pub enum Error {
    Generic(String),
    IO(io::Error),
    Utf8(string::FromUtf8Error),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Generic(_) => None,
            Self::IO(e) => Some(e),
            Self::Utf8(e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(s) => write!(f, "Error: {s}"),
            Self::IO(e) => write!(f, "IO Error: {e}"),
            Self::Utf8(e) => write!(f, "UTF-8 Error: {e}"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base = match self {
            Self::Generic(s) => s.clone(),
            Self::IO(e) => format!("{e:#?}"),
            Self::Utf8(e) => format!("{e:#?}"),
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
        Self::Utf8(e)
    }
}
