use std::fmt;

/// A single option.
///
/// For `Opt(x, y)`:
///   - `x` is the character representing the option.
///   - `y` is `Some` string, or `None` if no argument was expected.
///
/// # Example
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use getopt::Opt;
///
/// // args = ["program", "-abc", "foo"];
/// # let args: Vec<String> = vec!["program", "-abc", "foo"]
/// #     .into_iter()
/// #     .map(String::from)
/// #     .collect();
/// let optstring = "ab:c";
/// let mut opts = getopt::Parser::new(&args, optstring);
///
/// assert_eq!(Opt('a', None), opts.next().transpose()?.unwrap());
/// assert_eq!(Opt('b', Some("c".to_string())), opts.next().transpose()?.unwrap());
/// assert_eq!(None, opts.next().transpose()?);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Opt(pub char, pub Option<String>);

impl fmt::Display for Opt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opt({:?}, {:?})", self.0, self.1)
    }
}
