use std::{error, fmt};

use crate::ErrorKind::{self, *};

/// A basic error type for [`Parser`](struct.Parser.html)
#[derive(Debug, Eq, PartialEq)]
pub struct Error {
    culprit: char,
    kind: ErrorKind,
}

impl Error {
    /// Creates a new error using a known kind and the character that caused the issue.
    pub fn new(kind: ErrorKind, culprit: char) -> Self {
        Self { culprit, kind }
    }

    /// Returns the [`ErrorKind`](enum.ErrorKind.html) for this error.
    pub fn kind(self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            MissingArgument => write!(f, "option requires an argument -- {:?}", self.culprit),
            UnknownOption => write!(f, "unknown option -- {:?}", self.culprit),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
