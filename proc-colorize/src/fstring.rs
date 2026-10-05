//! Scanning of [`format!`](std::format!) style format strings.

use std::ops::Range;

/// The argument a placeholder refers to.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ArgRef<'a> {
    /// A positional argument, either implicit (`{}`) or explicit (`{0}`).
    Index(usize),
    /// A named argument, either captured inline (`{name}`) or passed as `name = value`.
    Name(&'a str),
}

/// A `{...}` placeholder within a format string.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Placeholder<'a> {
    /// Byte range of the placeholder, including its braces.
    pub range: Range<usize>,
    pub arg: ArgRef<'a>,
}

/// Finds every placeholder in `fstring`, skipping escaped `{{` and `}}` braces.
pub(crate) fn placeholders(fstring: &str) -> Result<Vec<Placeholder<'_>>, &'static str> {
    let bytes = fstring.as_bytes();
    let mut found = vec![];
    let mut next_positional = 0;
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'{' if bytes.get(i + 1) == Some(&b'{') => i += 2,
            b'}' if bytes.get(i + 1) == Some(&b'}') => i += 2,
            b'{' => {
                let close = match fstring[i + 1..].find(['{', '}']) {
                    Some(j) if bytes[i + 1 + j] == b'}' => i + 1 + j,
                    _ => {
                        return Err(
                            "invalid format string: unclosed `{`, use `{{` for a literal brace",
                        )
                    }
                };
                let inner = &fstring[i + 1..close];
                let (arg, spec) = inner.split_once(':').unwrap_or((inner, ""));

                // `.*` takes its precision from the next positional argument
                if spec.contains(".*") {
                    next_positional += 1;
                }

                let arg = if arg.is_empty() {
                    next_positional += 1;
                    ArgRef::Index(next_positional - 1)
                } else if let Ok(n) = arg.parse() {
                    ArgRef::Index(n)
                } else {
                    ArgRef::Name(arg)
                };

                found.push(Placeholder {
                    range: i..close + 1,
                    arg,
                });
                i = close + 1;
            }
            b'}' => {
                return Err("invalid format string: unmatched `}`, use `}}` for a literal brace")
            }
            _ => i += 1,
        }
    }

    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::{placeholders, ArgRef};

    fn args(fstring: &str) -> Vec<ArgRef<'_>> {
        placeholders(fstring)
            .unwrap()
            .into_iter()
            .map(|p| p.arg)
            .collect()
    }

    #[test]
    fn implicit_positional() {
        assert_eq!(
            args("{} {:?} {:>5}"),
            [ArgRef::Index(0), ArgRef::Index(1), ArgRef::Index(2)]
        );
    }

    #[test]
    fn placeholder_ranges() {
        let found = placeholders("a {} b {:?}").unwrap();
        assert_eq!(found[0].range, 2..4);
        assert_eq!(found[1].range, 7..11);
    }

    #[test]
    fn escaped_braces_are_not_placeholders() {
        assert_eq!(args("{{literal}}"), []);
        assert_eq!(args("{{{}}}"), [ArgRef::Index(0)]);
        assert_eq!(args("{{}} {}"), [ArgRef::Index(0)]);
    }

    #[test]
    fn named_args_do_not_consume_positional() {
        assert_eq!(
            args("{n} {} {name:?} {}"),
            [
                ArgRef::Name("n"),
                ArgRef::Index(0),
                ArgRef::Name("name"),
                ArgRef::Index(1)
            ]
        );
    }

    #[test]
    fn explicit_index() {
        assert_eq!(
            args("{1} {} {0:?}"),
            [ArgRef::Index(1), ArgRef::Index(0), ArgRef::Index(0)]
        );
    }

    #[test]
    fn star_precision_consumes_positional() {
        assert_eq!(args("{:.*} {}"), [ArgRef::Index(1), ArgRef::Index(2)]);
    }

    #[test]
    fn unicode_text() {
        let found = placeholders("héllo {} wörld").unwrap();
        assert_eq!(found[0].range, 7..9);
    }

    #[test]
    fn unbalanced_braces() {
        assert!(placeholders("{").is_err());
        assert!(placeholders("{ {}").is_err());
        assert!(placeholders("}").is_err());
        assert!(placeholders("{}}").is_err());
    }
}
