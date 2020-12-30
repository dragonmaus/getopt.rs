use std::collections::HashMap;

use crate::{error::Error, errorkind::ErrorKind, opt::Opt, result::Result};

/// The core of the `getopt` crate.
///
/// `Parser` is implemented as an iterator over the options present in the given argument vector.
///
/// The method [`next`](#method.next) does the heavy lifting.
///
/// # Examples
///
/// ## Simplified usage:
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use getopt::Opt;
///
/// // args = ["program", "-abc", "foo"];
/// # let args: Vec<String> = vec!["program", "-abc", "foo"]
/// #     .into_iter()
/// #     .map(String::from)
/// #     .collect();
/// let mut opts = getopt::Parser::new(&args, "ab:c");
///
/// assert_eq!(Some(Opt('a', None)), opts.next().transpose()?);
/// assert_eq!(1, opts.index());
/// assert_eq!(Some(Opt('b', Some("c".to_string()))), opts.next().transpose()?);
/// assert_eq!(2, opts.index());
/// assert_eq!(None, opts.next());
/// assert_eq!(2, opts.index());
/// assert_eq!("foo", args[opts.index()]);
/// # Ok(())
/// # }
/// ```
///
/// ## A more idiomatic example:
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use getopt::Opt;
///
/// // args = ["program", "-abc", "-d", "foo", "-e", "bar"];
/// # let mut args: Vec<String> = vec!["program", "-abc", "-d", "foo", "-e", "bar"]
/// #     .into_iter()
/// #     .map(String::from)
/// #     .collect();
/// let mut opts = getopt::Parser::new(&args, "ab:cd:e");
///
/// let mut a_flag = false;
/// let mut b_flag = String::new();
/// let mut c_flag = false;
/// let mut d_flag = String::new();
/// let mut e_flag = false;
///
/// loop {
///     match opts.next().transpose()? {
///         None => break,
///         Some(opt) => match opt {
///             Opt('a', None) => a_flag = true,
///             Opt('b', Some(arg)) => b_flag = arg.clone(),
///             Opt('c', None) => c_flag = true,
///             Opt('d', Some(arg)) => d_flag = arg.clone(),
///             Opt('e', None) => e_flag = true,
///             _ => unreachable!(),
///         },
///     }
/// }
///
/// let new_args = args.split_off(opts.index());
///
/// assert_eq!(true, a_flag);
/// assert_eq!("c", b_flag);
/// assert_eq!(false, c_flag);
/// assert_eq!("foo", d_flag);
/// assert_eq!(true, e_flag);
///
/// assert_eq!(1, new_args.len());
/// assert_eq!("bar", new_args.first().unwrap());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct Parser {
    opts: HashMap<char, bool>,
    args: Vec<Vec<char>>,
    index: usize,
    point: usize,
}

impl Parser {
    /// Create a new `Parser`, which will process the arguments in `args` according to the options
    /// specified in `optstring`.
    ///
    /// For compatibility with [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html),
    /// valid options are expected to begin at the second element of `args`, and `index` is
    /// initialised to `1`.
    /// If `args` is structured differently, be sure to call [`set_index`](#method.set_index)
    /// before the first invocation of [`next`](#method.next).
    ///
    /// `optstring` is a string of recognised option characters; if a character is followed by a
    /// colon (`:`), that option takes an argument.
    ///
    /// # Note:
    /// Transforming the OS-specific argument strings into a vector of `String`s is the sole
    /// responsibility of the calling program, as it involves some level of potential information
    /// loss (which this crate does not presume to handle unilaterally) and error handling (which
    /// would complicate the interface).
    pub fn new(args: &[String], optstring: &str) -> Self {
        let optstring: Vec<char> = optstring.chars().collect();
        let mut opts = HashMap::new();
        let mut i = 0;
        let len = optstring.len();

        while i < len {
            let j = i + 1;

            if j < len && optstring[j] == ':' {
                opts.insert(optstring[i], true);
                i += 1;
            } else {
                opts.insert(optstring[i], false);
            }
            i += 1;
        }

        Self {
            opts,
            // "explode" the args into a vector of character vectors, to allow indexing
            args: args.iter().map(|e| e.chars().collect()).collect(),
            index: 1,
            point: 0,
        }
    }

    /// Return the current `index` of the parser.
    ///
    /// `args[index]` will always point to the the next element of `args`; when the parser is
    /// finished with an element, it will increment `index`.
    ///
    /// After the last option has been parsed (and [`next`](#method.next) is returning `None`),
    /// `index` will point to the first non-option argument.
    pub fn index(&self) -> usize {
        self.index
    }

    // `point` must be reset to 0 whenever `index` is changed

    /// Modify the current `index` of the parser.
    pub fn set_index(&mut self, value: usize) {
        self.index = value;
        self.point = 0;
    }

    /// Increment the current `index` of the parser.
    ///
    /// This use case is common enough to warrant its own optimised method.
    pub fn incr_index(&mut self) {
        self.index += 1;
        self.point = 0;
    }
}

impl Iterator for Parser {
    type Item = Result<Opt>;

    /// Returns the next option, if any.
    ///
    /// Returns an [`Error`](struct.Error.html) if an unexpected option is encountered or if an
    /// expected argument is not found.
    ///
    /// Parsing stops at the first non-hyphenated argument; or at the first argument matching "-";
    /// or after the first argument matching "--".
    ///
    /// When no more options are available, `next` returns `None`.
    ///
    /// # Examples
    ///
    /// ## "-"
    /// ```
    /// use getopt::Parser;
    ///
    /// // args = ["program", "-", "-a"];
    /// # let args: Vec<String> = vec!["program", "-", "-a"]
    /// #     .into_iter()
    /// #     .map(String::from)
    /// #     .collect();
    /// let mut opts = Parser::new(&args, "a");
    ///
    /// assert_eq!(None, opts.next());
    /// assert_eq!("-", args[opts.index()]);
    /// ```
    ///
    /// ## "--"
    /// ```
    /// use getopt::Parser;
    ///
    /// // args = ["program", "--", "-a"];
    /// # let args: Vec<String> = vec!["program", "--", "-a"]
    /// #     .into_iter()
    /// #     .map(String::from)
    /// #     .collect();
    /// let mut opts = Parser::new(&args, "a");
    ///
    /// assert_eq!(None, opts.next());
    /// assert_eq!("-a", args[opts.index()]);
    /// ```
    ///
    /// ## Unexpected option:
    /// ```
    /// use getopt::Parser;
    ///
    /// // args = ["program", "-b"];
    /// # let args: Vec<String> = vec!["program", "-b"]
    /// #     .into_iter()
    /// #     .map(String::from)
    /// #     .collect();
    /// let mut opts = Parser::new(&args, "a");
    ///
    /// assert_eq!(
    ///     "unknown option -- 'b'".to_string(),
    ///     opts.next().unwrap().unwrap_err().to_string()
    /// );
    /// ```
    ///
    /// ## Missing argument:
    /// ```
    /// use getopt::Parser;
    ///
    /// // args = ["program", "-a"];
    /// # let args: Vec<String> = vec!["program", "-a"]
    /// #     .into_iter()
    /// #     .map(String::from)
    /// #     .collect();
    /// let mut opts = Parser::new(&args, "a:");
    ///
    /// assert_eq!(
    ///     "option requires an argument -- 'a'".to_string(),
    ///     opts.next().unwrap().unwrap_err().to_string()
    /// );
    /// ```
    fn next(&mut self) -> Option<Result<Opt>> {
        if self.point == 0 {
            /*
             * Rationale excerpts below taken verbatim from "The Open Group Base Specifications
             * Issue 7, 2018 edition", IEEE Std 1003.1-2017 (Revision of IEEE Std 1003.1-2008).
             * Copyright © 2001-2018 IEEE and The Open Group.
             */

            /*
             * If, when getopt() is called:
             *      argv[optind]    is a null pointer
             *      *argv[optind]   is not the character '-'
             *      argv[optind]    points to the string "-"
             * getopt() shall return -1 without changing optind.
             */
            if self.index >= self.args.len()
                || self.args[self.index].is_empty()
                || self.args[self.index][0] != '-'
                || self.args[self.index].len() == 1
            {
                return None;
            }

            /*
             * If:
             *      argv[optind]    points to the string "--"
             * getopt() shall return -1 after incrementing index.
             */
            if self.args[self.index][1] == '-' && self.args[self.index].len() == 2 {
                self.incr_index();
                return None;
            }

            // move past the starting '-'
            self.point += 1;
        }

        let opt = self.args[self.index][self.point];
        self.point += 1;

        match self.opts.get(&opt) {
            None => {
                if self.point >= self.args[self.index].len() {
                    self.incr_index();
                }
                Some(Err(Error::new(ErrorKind::UnknownOption, opt)))
            }
            Some(false) => {
                if self.point >= self.args[self.index].len() {
                    self.incr_index();
                }

                Some(Ok(Opt(opt, None)))
            }
            Some(true) => {
                let arg: String = if self.point >= self.args[self.index].len() {
                    self.incr_index();
                    if self.index >= self.args.len() {
                        return Some(Err(Error::new(ErrorKind::MissingArgument, opt)));
                    }
                    self.args[self.index].iter().collect()
                } else {
                    self.args[self.index]
                        .clone()
                        .split_off(self.point)
                        .iter()
                        .collect()
                };

                self.incr_index();

                Some(Ok(Opt(opt, Some(arg))))
            }
        }
    }
}
