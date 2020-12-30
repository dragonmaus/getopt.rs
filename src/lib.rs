#![doc(html_root_url = "https://docs.rs/getopt/1.0.3")]

//! # getopt
//!
//! `getopt` provides a minimal, (essentially) POSIX-compliant option parser.

pub use crate::{error::Error, errorkind::ErrorKind, opt::Opt, parser::Parser, result::Result};

pub mod prelude;

mod error;
mod errorkind;
mod opt;
mod parser;
mod result;
#[cfg(test)]
mod tests;

// Include README.md when running doctests.
// Credit to Guillaume Gomez (https://github.com/GuillaumeGomez/doc-comment) for the idea and
// implementation.
// TODO: Once RFC 1990 (issue #44732) lands, change this to:
// #[doc(include = "../README.md")]
macro_rules! doc_comment {
    ($x:expr, $y:ident) => {
        #[doc = $x]
        mod $y {}
    };
}
doc_comment!(include_str!("../README.md"), readme);
