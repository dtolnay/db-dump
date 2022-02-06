use std::fmt::{self, Debug, Display};
use std::io;

/// Error type returned by `db_dump::load_all` and `Loader::load` in the event
/// that loading crates.io's DB dump from the specified file fails.
pub struct Error {
    e: Box<ErrorImpl>,
}

/// Result type returned by `db_dump::load_all` and `Loader::load`.
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) enum ErrorImpl {
    Msg(String),
    Csv(csv::Error),
    Io(io::Error),
    Json(serde_json::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self.e {
            ErrorImpl::Msg(e) => f.write_str(e),
            ErrorImpl::Io(e) => Display::fmt(e, f),
            ErrorImpl::Csv(e) => Display::fmt(e, f),
            ErrorImpl::Json(e) => Display::fmt(e, f),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "db_dump::Error({:?})", self.to_string())
    }
}

pub(crate) fn err(variant: impl Into<ErrorImpl>) -> Error {
    Error {
        e: Box::new(variant.into()),
    }
}

impl<'a> From<fmt::Arguments<'a>> for ErrorImpl {
    fn from(e: fmt::Arguments) -> Self {
        ErrorImpl::Msg(e.to_string())
    }
}

impl From<csv::Error> for ErrorImpl {
    fn from(e: csv::Error) -> Self {
        ErrorImpl::Csv(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            e: Box::new(ErrorImpl::Io(e)),
        }
    }
}

impl From<serde_json::Error> for ErrorImpl {
    fn from(e: serde_json::Error) -> Self {
        ErrorImpl::Json(e)
    }
}
