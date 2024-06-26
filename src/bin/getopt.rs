extern crate getopt;

use std::{io, process};

use getopt::Opt;

// Command-line program boilerplate
mod program {
    use std::{env, io, path::Path};

    pub use self::Result::*;

    pub enum Result {
        Ok(i32),
        External(io::Error),
        Internal(io::Error),
    }

    pub fn args() -> Vec<String> {
        env::args_os()
            .map(|a| a.to_string_lossy().into_owned())
            .collect()
    }

    pub fn name(default: &str) -> String {
        match env::args_os().next() {
            None => String::from(default),
            Some(os_string) => match Path::new(&os_string).file_stem() {
                None => String::from(default),
                Some(os_str) => os_str.to_string_lossy().into_owned(),
            },
        }
    }
}

enum ShellKind {
    Bourne,
    C,
    Fish,
    Rc,
}

fn main() -> ! {
    process::exit(match program(&program::name("getopt")) {
        program::Ok(code) => code,
        program::External(error) => {
            eprintln!("{}", error);
            1
        },
        program::Internal(error) => {
            eprintln!("{}", error);
            2
        },
    });
}

#[rustfmt::skip]
fn print_usage(program_name: &str) {
    println!("Usage: {} [-h] [-n name] [-s shell] optstring [args ...]", program_name);
    println!("  -n name   report errors as 'name' (default '{}')", program_name);
    println!("  -s shell  use quoting conventions for shell (default 'sh')");
    println!();
    println!("  -h        display this help");
}

fn program(name: &str) -> program::Result {
    let args = program::args();
    let mut parsed: Vec<String> = Vec::new();

    let mut child_name = name.to_string();
    let mut shell = ShellKind::Bourne;

    // gather our own options
    let mut opts = getopt::Parser::new(&args, "hn:s:");
    loop {
        match opts.next() {
            None => break,
            Some(Err(error)) => {
                return program::Internal(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{}: {}", name, error),
                ));
            },
            Some(Ok(opt)) => match opt {
                Opt('n', Some(arg)) => child_name = arg,
                Opt('s', Some(arg)) => {
                    shell = match arg.to_lowercase().trim() {
                        "ash" | "bash" | "dash" | "ksh" | "mksh" | "sh" | "zsh" => {
                            ShellKind::Bourne
                        },
                        "csh" | "tcsh" => ShellKind::C,
                        "fish" => ShellKind::Fish,
                        "plan9" | "rc" => ShellKind::Rc,
                        x => {
                            return program::Internal(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                format!("{}: unknown shell type: {}", name, x),
                            ));
                        },
                    }
                },
                Opt('h', None) => {
                    print_usage(name);
                    return program::Ok(0);
                },
                _ => unreachable!(),
            },
        }
    }

    let optstring = match args.get(opts.index()) {
        None => {
            return program::Internal(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{}: missing optstring argument", name),
            ));
        },
        Some(s) => s,
    };
    let index = opts.index() + 1;

    // parse the other options
    let mut opts = getopt::Parser::new(&args, optstring);
    opts.set_index(index);
    loop {
        match opts.next() {
            None => break,
            Some(Err(error)) => {
                return program::External(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{}: {}", child_name, error),
                ));
            },
            Some(Ok(Opt(opt, arg))) => {
                parsed.push(format!("-{}", opt));
                match arg {
                    None => (),
                    Some(s) => parsed.push(quote_for_shell(&s, &shell)),
                }
            },
        }
    }

    parsed.push("--".to_string());

    for arg in args.clone().split_off(opts.index()) {
        parsed.push(quote_for_shell(&arg, &shell));
    }

    println!("{}", parsed.join(" "));

    program::Ok(0)
}

fn quote_for_shell(string: &str, kind: &ShellKind) -> String {
    match kind {
        // most shells (sh, ksh, zsh, bash, (d)ash, etc.) are in this category
        ShellKind::Bourne => {
            let e = '\\'; // escape char
            let q = '\''; // quote char
            let mut new_string = String::new();
            new_string.push(q);
            for c in string.chars() {
                match c {
                    '\'' => {
                        new_string.push(q);
                        new_string.push(e);
                        new_string.push(c);
                        new_string.push(q);
                    },
                    _ => new_string.push(c),
                }
            }
            new_string.push(q);
            new_string
        },

        ShellKind::C => {
            let e = '\\'; // escape char
            let q = '\''; // quote char
            let mut new_string = String::new();
            new_string.push(q);
            for c in string.chars() {
                match c {
                    ' ' | '\'' => {
                        new_string.push(q);
                        new_string.push(e);
                        new_string.push(c);
                        new_string.push(q);
                    },
                    _ => new_string.push(c),
                }
            }
            new_string.push(q);
            new_string
        },

        ShellKind::Fish => {
            let e = '\\'; // escape char
            let q = '\''; // quote char
            let mut new_string = String::new();
            new_string.push(q);
            for c in string.chars() {
                match c {
                    '\'' | '\\' => {
                        new_string.push(e);
                        new_string.push(c);
                    },
                    _ => new_string.push(c),
                }
            }
            new_string.push(q);
            new_string
        },

        ShellKind::Rc => {
            let q = '\''; // quote char
            let mut new_string = String::new();
            new_string.push(q);
            for c in string.chars() {
                match c {
                    '\'' => {
                        new_string.push(q);
                        new_string.push(c);
                    },
                    _ => new_string.push(c),
                }
            }
            new_string.push(q);
            new_string
        },
    }
}
