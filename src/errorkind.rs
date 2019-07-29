/// What kinds of errors [`Parser`](struct.Parser.html) can return.
#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// An argument was not found for an option that was expecting one.
    MissingArgument,
    /// An unknown option character was encountered.
    UnknownOption,
}
