use crate::{Opt, Parser};

macro_rules! basic_test {
    ($name:ident, $expect:expr, $next:expr, [$($arg:expr),+], $optstr:expr) => (
        #[test]
        fn $name() -> Result<(), String> {
            let expect: Option<Opt> = $expect;
            let args: Vec<String> = vec![$($arg),+].into_iter().map(String::from).collect();
            let next: Option<String> = $next;
            let mut opts = Parser::new(&args, $optstr);

            match opts.next().transpose() {
                Err(error) => {
                    return Err(format!("next() returned {:?}", error))
                },
                Ok(actual) => if actual != expect {
                    return Err(format!("expected {:?}; got {:?}", expect, actual))
                },
            };

            match next {
                None => if opts.index() < args.len() {
                    return Err(format!("expected end of args; got {:?}", args[opts.index()]))
                },
                Some(n) => if args[opts.index()] != n {
                    return Err(format!("next arg: expected {:?}; got {:?}", n, args[opts.index()]))
                },
            };

            Ok(())
        }
    )
}

#[rustfmt::skip] basic_test!(blank_arg, None, Some(String::new()), ["x", ""], "a");
#[rustfmt::skip] basic_test!(double_dash, None, Some("-a".to_string()), ["x", "--", "-a", "foo"], "a");
#[rustfmt::skip] basic_test!(no_opts_1, None, None, ["x"], "a");
#[rustfmt::skip] basic_test!(no_opts_2, None, Some("foo".to_string()), ["x", "foo"], "a");
#[rustfmt::skip] basic_test!(no_opts_3, None, Some("foo".to_string()), ["x", "foo", "-a"], "a");
#[rustfmt::skip] basic_test!(single_dash, None, Some("-".to_string()), ["x", "-", "-a", "foo"], "a");
#[rustfmt::skip] basic_test!(single_opt, Some(Opt('a', None)), Some("foo".to_string()), ["x", "-a", "foo"], "a");
#[rustfmt::skip] basic_test!(single_optarg, Some(Opt('a', Some("foo".to_string()))), None, ["x", "-a", "foo"], "a:");

macro_rules! error_test {
    ($name:ident, $expect:expr, [$($arg:expr),+], $optstr:expr) => (
        #[test]
        fn $name() -> Result<(), String> {
            let expect: String = $expect.to_string();
            let args: Vec<String> = vec![$($arg),+].into_iter().map(String::from).collect();
            let mut opts = Parser::new(&args, $optstr);

            match opts.next() {
                None => {
                    return Err(format!("unexpected successful response: end of options"))
                },
                Some(Err(actual)) => {
                    let actual = actual.to_string();

                    if actual != expect {
                        return Err(format!("expected {:?}; got {:?}", expect, actual));
                    }
                },
                Some(Ok(opt)) => {
                    return Err(format!("unexpected successful response: {:?}", opt))
                },
            };

            Ok(())
        }
    )
}

#[rustfmt::skip] error_test!(bad_opt, "unknown option -- 'b'", ["x", "-b"], "a");
#[rustfmt::skip] error_test!(missing_optarg, "option requires an argument -- 'a'", ["x", "-a"], "a:");

#[test]
fn multiple() -> Result<(), String> {
    let args: Vec<String> = vec!["x", "-abc", "-d", "foo", "-e", "bar"]
        .into_iter()
        .map(String::from)
        .collect();
    let optstring = "ab:d:e".to_string();
    let mut opts = Parser::new(&args, &optstring);

    macro_rules! check_result {
        ($expect:expr) => {
            let expect: Option<Opt> = $expect;
            match opts.next().transpose() {
                Err(error) => return Err(format!("next() returned {:?}", error)),
                Ok(actual) => {
                    if actual != expect {
                        return Err(format!("expected {:?}; got {:?}", expect, actual));
                    }
                },
            };
        };
    }

    check_result!(Some(Opt('a', None)));
    check_result!(Some(Opt('b', Some("c".to_string()))));
    check_result!(Some(Opt('d', Some("foo".to_string()))));
    check_result!(Some(Opt('e', None)));
    check_result!(None);

    Ok(())
}

#[test]
fn continue_after_error() {
    let args: Vec<String> = vec!["x", "-z", "-abc"]
        .into_iter()
        .map(String::from)
        .collect();
    let optstring = "ab:d:e".to_string();
    for _opt in Parser::new(&args, &optstring) {
        // do nothing, should not panic
    }
}
