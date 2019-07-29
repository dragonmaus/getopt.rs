use std::result;

use crate::error::Error;

/// A specialized `Result` type for use with [`Parser`](struct.Parser.html)
pub type Result<T> = result::Result<T, Error>;
