
#![allow(dead_code)]

use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::path::{Path, PathBuf};
use std::result;

/// A type alias for handling errors throughout gli-rs.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur.
#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }

    pub fn parse<T: AsRef<str>>(msg: T) -> Error {
        Error::from(ErrorKind::Parse(msg.as_ref().to_string()))
    }

    pub fn bug<T: AsRef<str>>(msg: T) -> Error {
        Error::from(ErrorKind::Bug(msg.as_ref().to_string()))
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

/// The specific kind of error that can occur.
#[derive(Clone, Debug)]
pub enum ErrorKind {

    /// An error that occurred while parsing a data source
    Parse(String),

    /// An error that occurred while working with a file path.
    Path(PathBuf),

    /// Generally, these errors correspond to bugs in this library.
    Bug(String),

    /// An unexpected I/O error occurred.
    Io,
}

impl ErrorKind {

    /// A convenience routine for creating an error associated with a path.
    pub fn path(path: impl AsRef<Path>) -> ErrorKind {
        ErrorKind::Path(path.as_ref().to_path_buf())
    }
}

impl fmt::Display for ErrorKind {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match *self {
            | ErrorKind::Parse(ref msg) => {
                write!(f, "Parse error: {}", msg)
            },
            | ErrorKind::Path(ref path) => {
                write!(f, "{}", path.display())
            },
            | ErrorKind::Bug(ref msg) => {
                let report = "Please report this bug with a backtrace for this repository";
                write!(f, "Bug: {}\n{}", msg, report)
            },
            | ErrorKind::Io => {
                write!(f, "I/O error")
            },
        }
    }
}

impl From<ErrorKind> for Error {

    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {

    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx }
    }
}
