use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::none_of,
    combinator::{map, peek, recognize},
    multi::{count, many_till},
    sequence::{delimited, preceded},
};
use prettify::{concat, string as prettify_string, PrettifyDoc};

#[derive(PartialEq, Debug, Clone)]
pub enum StringFragment<'a> {
    Unescaped(&'a str),
    EscapedUnicode(&'a str),
    Escaped(&'a str),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct StringOptions<'a> {
    pub backslash_escaped_characters: &'a str,
    pub allow_line_breaks: bool,
}

pub fn unicode_escape_sequence(input: &str) -> nom::IResult<&str, StringFragment> {
    let (remainder, value) = alt((
        preceded(tag("\\u"), recognize(count(none_of("\n\r"), 4))),
        preceded(tag("\\U"), recognize(count(none_of("\n\r"), 8))),
    ))(input)?;
    Ok((remainder, StringFragment::EscapedUnicode(value)))
}

pub fn backslash_escape(input: &str) -> nom::IResult<&str, StringFragment> {
    map(
        preceded(tag("\\"), recognize(none_of("\n\r"))),
        StringFragment::Escaped,
    )(input)
}

pub fn unescaped_char(input: &str) -> nom::IResult<&str, StringFragment> {
    map(recognize(none_of("\n\r")), StringFragment::Unescaped)(input)
}

pub fn unescaped_char_multiline(input: &str) -> nom::IResult<&str, StringFragment> {
    map(take(1 as usize), StringFragment::Unescaped)(input)
}

pub fn parse_custom_quoted_string<'a>(
    quote: &'a str,
    options: StringOptions<'a>,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, Vec<StringFragment<'a>>> {
    delimited(
        tag(quote),
        map(
            many_till(
                alt((
                    unicode_escape_sequence,
                    backslash_escape,
                    if options.allow_line_breaks {
                        unescaped_char_multiline
                    } else {
                        unescaped_char
                    },
                )),
                peek(tag(quote)),
            ),
            |result| result.0,
        ),
        tag(quote),
    )
}

pub fn format_custom_quoted_string<'a>(
    quote: &'a str,
    fragments: Vec<StringFragment<'a>>,
    options: StringOptions<'a>,
) -> PrettifyDoc<'a> {
    concat(vec![
        prettify_string(quote),
        concat(
            fragments
                .iter()
                .map(|fragment| match *fragment {
                    StringFragment::Unescaped(value) => prettify_string(value),
                    StringFragment::EscapedUnicode(value) => concat(vec![
                        prettify_string(if value.len() == 4 { "\\u" } else { "\\U" }),
                        prettify_string(value),
                    ]),
                    StringFragment::Escaped(value) => {
                        if value == quote
                            || value == "\\"
                            || options.backslash_escaped_characters.contains(value)
                        {
                            concat(vec![prettify_string("\\"), prettify_string(value)])
                        } else {
                            prettify_string(value)
                        }
                    }
                })
                .collect(),
        ),
        prettify_string(quote),
    ])
}

pub fn custom_quoted_string<'a>(
    quote: &'a str,
    options: StringOptions<'a>,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    map(parse_custom_quoted_string(quote, options), move |result| {
        format_custom_quoted_string(quote, result, options)
    })
}

pub fn parse_single_quoted_string<'a>(
    options: StringOptions<'a>,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, Vec<StringFragment<'a>>> {
    parse_custom_quoted_string("'", options)
}

pub fn format_single_quoted_string<'a>(
    fragments: Vec<StringFragment<'a>>,
    options: StringOptions<'a>,
) -> PrettifyDoc<'a> {
    format_custom_quoted_string("'", fragments, options)
}

pub fn single_quoted_string<'a>(
    options: StringOptions<'a>,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    map(parse_single_quoted_string(options), move |result| {
        format_single_quoted_string(result, options)
    })
}

pub fn parse_double_quoted_string<'a>(
    options: StringOptions<'a>,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, Vec<StringFragment<'a>>> {
    parse_custom_quoted_string("\"", options)
}

pub fn format_double_quoted_string<'a>(
    fragments: Vec<StringFragment<'a>>,
    options: StringOptions<'a>,
) -> PrettifyDoc<'a> {
    format_custom_quoted_string("\"", fragments, options)
}

pub fn double_quoted_string<'a>(
    options: StringOptions<'a>,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    map(parse_double_quoted_string(options), move |result| {
        format_double_quoted_string(result, options)
    })
}

pub fn parse_string<'a>(
    options: StringOptions<'a>,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, Vec<StringFragment<'a>>> {
    alt((
        parse_single_quoted_string(options),
        parse_double_quoted_string(options),
    ))
}

pub fn format_string<'a>(
    fragments: Vec<StringFragment<'a>>,
    options: StringOptions<'a>,
) -> PrettifyDoc<'a> {
    let mut single_quote_count = 0;
    let mut double_quote_count = 0;
    for fragment in fragments.iter() {
        if let StringFragment::Escaped(value) = fragment {
            if *value == "'" {
                single_quote_count += 1;
            } else if *value == "\"" {
                double_quote_count += 1;
            }
        }
    }
    if single_quote_count < double_quote_count {
        format_single_quoted_string(fragments, options)
    } else {
        format_double_quoted_string(fragments, options)
    }
}

pub fn parse_and_format_string<'a>(
    options: StringOptions<'a>,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    map(parse_string(options), move |result| {
        format_string(result, options)
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_errors, assert_formatted};

    #[test]
    fn test_unicode_escape_sequence() {
        assert_eq!(
            unicode_escape_sequence("\\u1234"),
            Ok(("", StringFragment::EscapedUnicode("1234")))
        );
        assert_eq!(
            unicode_escape_sequence("\\U12345678"),
            Ok(("", StringFragment::EscapedUnicode("12345678")))
        );

        assert_errors(unicode_escape_sequence("\\u123\n"));
        assert_errors(unicode_escape_sequence("\\u123\r"));
        assert_errors(unicode_escape_sequence("\\U1234567\n"));
        assert_errors(unicode_escape_sequence("\\U1234567\r"));
    }

    #[test]
    fn test_backslash_escaped() {
        assert_eq!(
            backslash_escape("\\'"),
            Ok(("", StringFragment::Escaped("'")))
        );
        assert_eq!(
            backslash_escape("\\b\\'"),
            Ok(("\\'", StringFragment::Escaped("b")))
        );
        assert_eq!(
            backslash_escape("\\'\\b"),
            Ok(("\\b", StringFragment::Escaped("'")))
        );

        assert_errors(backslash_escape("\\\n"));
        assert_errors(backslash_escape("\\\r"));
    }

    #[test]
    fn test_unescaped_char() {
        assert_eq!(
            unescaped_char("a"),
            Ok(("", StringFragment::Unescaped("a")))
        );
        assert_eq!(
            unescaped_char("ab"),
            Ok(("b", StringFragment::Unescaped("a")))
        );

        assert_errors(unescaped_char("\n"));
        assert_errors(unescaped_char("\r"));
    }

    #[test]
    fn test_unescaped_char_multiline() {
        assert_eq!(
            unescaped_char_multiline("a"),
            Ok(("", StringFragment::Unescaped("a")))
        );
        assert_eq!(
            unescaped_char_multiline("ab"),
            Ok(("b", StringFragment::Unescaped("a")))
        );
        assert_eq!(
            unescaped_char_multiline("\n"),
            Ok(("", StringFragment::Unescaped("\n")))
        );
        assert_eq!(
            unescaped_char_multiline("\r"),
            Ok(("", StringFragment::Unescaped("\r")))
        );
    }

    #[test]
    fn test_parse_single_quoted_string() {
        let options = StringOptions {
            allow_line_breaks: false,
            backslash_escaped_characters: "",
        };
        assert_eq!(
            parse_single_quoted_string(options)("'a'"),
            Ok(("", vec![StringFragment::Unescaped("a")]))
        );
        assert_eq!(
            parse_single_quoted_string(options)("'a\\b'"),
            Ok((
                "",
                vec![StringFragment::Unescaped("a"), StringFragment::Escaped("b")]
            ))
        );
        assert_eq!(
            parse_single_quoted_string(options)("'a\\b\\''"),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::Escaped("b"),
                    StringFragment::Escaped("'"),
                ]
            ))
        );
        assert_eq!(
            parse_single_quoted_string(options)("'a\\u1234'"),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::EscapedUnicode("1234"),
                ]
            ))
        );
    }

    #[test]
    fn test_parse_double_quoted_string() {
        let options = StringOptions {
            allow_line_breaks: false,
            backslash_escaped_characters: "",
        };
        assert_eq!(
            parse_double_quoted_string(options)("\"a\""),
            Ok(("", vec![StringFragment::Unescaped("a")]))
        );
        assert_eq!(
            parse_double_quoted_string(options)("\"a\\b\""),
            Ok((
                "",
                vec![StringFragment::Unescaped("a"), StringFragment::Escaped("b")]
            ))
        );
        assert_eq!(
            parse_double_quoted_string(options)("\"a\\b\\\"\""),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::Escaped("b"),
                    StringFragment::Escaped("\""),
                ]
            ))
        );
        assert_eq!(
            parse_double_quoted_string(options)("\"a\\u1234\""),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::EscapedUnicode("1234"),
                ]
            ))
        );
    }

    #[test]
    fn test_parse_string() {
        let options = StringOptions {
            allow_line_breaks: false,
            backslash_escaped_characters: "",
        };
        assert_eq!(
            parse_string(options)("\"a\""),
            Ok(("", vec![StringFragment::Unescaped("a")]))
        );
        assert_eq!(
            parse_string(options)("'a'"),
            Ok(("", vec![StringFragment::Unescaped("a")]))
        );
        assert_eq!(
            parse_string(options)("'a\\b'"),
            Ok((
                "",
                vec![StringFragment::Unescaped("a"), StringFragment::Escaped("b")]
            ))
        );
        assert_eq!(
            parse_string(options)("'a\\b\\''"),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::Escaped("b"),
                    StringFragment::Escaped("'"),
                ]
            ))
        );
    }

    #[test]
    fn test_single_quoted_string() {
        assert_formatted(
            single_quoted_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("'a'"),
            ("", "'a'"),
        );
        assert_formatted(
            single_quoted_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("'\\a'"),
            ("", "'a'"),
        );
        assert_formatted(
            single_quoted_string(StringOptions {
                backslash_escaped_characters: "a",
                allow_line_breaks: false,
            })("'\\a'"),
            ("", "'\\a'"),
        );
        assert_formatted(
            single_quoted_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("'\\''"),
            ("", "'\\''"),
        );
    }

    #[test]
    fn test_double_quoted_string() {
        assert_formatted(
            double_quoted_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("\"a\""),
            ("", "\"a\""),
        );
        assert_formatted(
            double_quoted_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("\"\\a\""),
            ("", "\"a\""),
        );
        assert_formatted(
            double_quoted_string(StringOptions {
                backslash_escaped_characters: "a",
                allow_line_breaks: false,
            })("\"\\a\""),
            ("", "\"\\a\""),
        );
        assert_formatted(
            double_quoted_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("\"\\\"\""),
            ("", "\"\\\"\""),
        );
    }

    #[test]
    fn test_parse_and_format_string() {
        assert_formatted(
            parse_and_format_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("\"a\""),
            ("", "\"a\""),
        );
        assert_formatted(
            parse_and_format_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("'a'"),
            ("", "\"a\""),
        );
        assert_formatted(
            parse_and_format_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("'\\a'"),
            ("", "\"a\""),
        );
        assert_formatted(
            parse_and_format_string(StringOptions {
                backslash_escaped_characters: "a",
                allow_line_breaks: false,
            })("'\\a'"),
            ("", "\"\\a\""),
        );
        assert_formatted(
            parse_and_format_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("'\\''"),
            ("", "\"'\""),
        );
        assert_formatted(
            parse_and_format_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("'\\\"'"),
            ("", "'\"'"),
        );
        assert_formatted(
            parse_and_format_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("\"\\\"\""),
            ("", "'\"'"),
        );
        assert_formatted(
            parse_and_format_string(StringOptions {
                backslash_escaped_characters: "",
                allow_line_breaks: false,
            })("'\\\\'"),
            ("", "\"\\\\\""),
        );
    }
}
