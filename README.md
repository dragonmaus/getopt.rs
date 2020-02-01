# getopt

A minimal, (essentially) POSIX-compliant option parser.

`getopt::Parser` iterates over the provided arguments, producing options one at
a time in the order in which they are given on the command line, and stopping
at the first non-option argument.

## Example:
```rust
#![allow(unused_assignments, unused_variables)]

use getopt::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = std::env::args().collect();
    let mut opts = Parser::new(&args, "ab:");

    let mut a_flag = false;
    let mut b_flag = String::new();
    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('a', None) => a_flag = true,
                Opt('b', Some(string)) => b_flag = string.clone(),
                _ => unreachable!(),
            }
        }
    }

    let args = args.split_off(opts.index());

    // …

    Ok(())
}
```

## Links:
- [Crates.io](https://crates.io/crates/getopt)
- [Documentation](https://docs.rs/getopt/)
- [Repository](https://git.dragonma.us/dragonmaus/getopt.rs) ([GitLab](https://gitlab.com/dragonmaus/getopt.rs), [GitHub](https://github.com/dragonmaus/getopt.rs))
