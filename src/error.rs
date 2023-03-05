use std::fmt::{self, Debug, Display};
use std::io;
use std::path::Path;

/// Error type returned by `db_dump::load_all` and `Loader::load` in the event
/// that loading crates.io's DB dump from the specified file fails.
pub struct Error {
    pub(crate) e: Box<ErrorImpl>,
}

/// Result type returned by `db_dump::load_all` and `Loader::load`.
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) struct ErrorImpl {
    pub(crate) path: Option<&'static Path>,
    kind: ErrorKind,
}

pub(crate) enum ErrorKind {
    Msg(String),
    Csv(csv::Error),
    Io(io::Error),
    Json(serde_json::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.e.kind {
            ErrorKind::Msg(_) => None,
            ErrorKind::Io(e) => e.source(),
            ErrorKind::Csv(e) => e.source(),
            ErrorKind::Json(e) => e.source(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(path) = self.e.path {
            write!(f, "{}.csv: ", path.display())?;
        }
        match &self.e.kind {
            ErrorKind::Msg(e) => f.write_str(e),
            ErrorKind::Io(e) => write!(f, "{}", e),
            ErrorKind::Csv(e) => write!(f, "{}", e),
            ErrorKind::Json(e) => write!(f, "{}", e),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "db_dump::Error({:?})", self.to_string())
    }
}

pub(crate) fn err(variant: impl Into<ErrorKind>) -> Error {
    Error {
        e: Box::new(ErrorImpl {
            path: None,
            kind: variant.into(),
        }),
    }
}

impl<'a> From<fmt::Arguments<'a>> for ErrorKind {
    fn from(e: fmt::Arguments) -> Self {
        ErrorKind::Msg(e.to_string())
    }
}

impl From<csv::Error> for ErrorKind {
    fn from(e: csv::Error) -> Self {
        ErrorKind::Csv(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            e: Box::new(ErrorImpl {
                path: None,
                kind: ErrorKind::Io(e),
            }),
        }
    }
}

impl From<serde_json::Error> for ErrorKind {
    fn from(e: serde_json::Error) -> Self {
        ErrorKind::Json(e)
    }
}
