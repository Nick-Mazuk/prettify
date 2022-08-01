use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{none_of, one_of},
    combinator::{cond, map, map_res, peek, recognize},
    multi::{many_m_n, many_till},
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
pub enum QuoteType {
    Single,
    Double,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct StringOptions<'a> {
    backslash_escaped_characters: &'a str,
    allow_line_breaks: bool,
    preferred_quote_type: Option<QuoteType>,
    allow_unicode_4_digit_escape: bool,
    allow_unicode_8_digit_escape: bool,
    unicode_transform_lowercase: bool,
}

impl<'a> StringOptions<'a> {
    pub fn new() -> StringOptions<'a> {
        StringOptions {
            backslash_escaped_characters: "",
            allow_line_breaks: false,
            preferred_quote_type: None,
            allow_unicode_4_digit_escape: false,
            allow_unicode_8_digit_escape: false,
            unicode_transform_lowercase: false,
        }
    }

    pub fn backslash_escaped_characters(mut self, backslash_escaped_characters: &'a str) -> Self {
        self.backslash_escaped_characters = backslash_escaped_characters;
        self
    }

    pub fn allow_line_breaks(mut self) -> Self {
        self.allow_line_breaks = true;
        self
    }

    pub fn preferred_quote_type(mut self, quote_type: QuoteType) -> Self {
        self.preferred_quote_type = Some(quote_type);
        self
    }

    pub fn allow_unicode_4_digit_escape(mut self) -> Self {
        self.allow_unicode_4_digit_escape = true;
        self
    }

    pub fn allow_unicode_8_digit_escape(mut self) -> Self {
        self.allow_unicode_8_digit_escape = true;
        self
    }

    pub fn unicode_transform_lowercase(mut self) -> Self {
        self.unicode_transform_lowercase = true;
        self
    }
}

pub fn unicode_4_digit_escape_sequence(input: &str) -> nom::IResult<&str, StringFragment> {
    map(
        preceded(
            tag("\\u"),
            recognize(many_m_n(4, 4, one_of("0123456789abcdefABCDEF"))),
        ),
        StringFragment::EscapedUnicode,
    )(input)
}

pub fn unicode_8_digit_escape_sequence(input: &str) -> nom::IResult<&str, StringFragment> {
    map(
        preceded(
            tag("\\U"),
            recognize(many_m_n(8, 8, one_of("0123456789abcdefABCDEF"))),
        ),
        StringFragment::EscapedUnicode,
    )(input)
}

pub fn unicode_escape_sequence(input: &str) -> nom::IResult<&str, StringFragment> {
    alt((
        unicode_4_digit_escape_sequence,
        unicode_8_digit_escape_sequence,
    ))(input)
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
                    map_res(
                        cond(
                            options.allow_unicode_4_digit_escape,
                            unicode_4_digit_escape_sequence,
                        ),
                        |result| {
                            result.ok_or(nom::error::Error {
                                code: nom::error::ErrorKind::MapRes,
                                input: "",
                            })
                        },
                    ),
                    map_res(
                        cond(
                            options.allow_unicode_8_digit_escape,
                            unicode_8_digit_escape_sequence,
                        ),
                        |result| {
                            result.ok_or(nom::error::Error {
                                code: nom::error::ErrorKind::MapRes,
                                input: "",
                            })
                        },
                    ),
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
                        prettify_string(if options.unicode_transform_lowercase {
                            value.to_lowercase()
                        } else {
                            value.to_uppercase()
                        }),
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
    match options.preferred_quote_type {
        Some(QuoteType::Single) => format_single_quoted_string(fragments, options),
        Some(QuoteType::Double) => format_double_quoted_string(fragments, options),
        None => {
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
    fn test_unicode_4_digit_escape_sequence() {
        assert_eq!(
            unicode_4_digit_escape_sequence("\\u1234"),
            Ok(("", StringFragment::EscapedUnicode("1234")))
        );

        assert_errors(unicode_4_digit_escape_sequence("\\u123\n"));
        assert_errors(unicode_4_digit_escape_sequence("\\u123\r"));
        assert_errors(unicode_4_digit_escape_sequence("\\u123"));
        assert_errors(unicode_4_digit_escape_sequence("\\u122z"));
        assert_errors(unicode_4_digit_escape_sequence("\\U1223"));
    }

    #[test]
    fn test_unicode_8_digit_escape_sequence() {
        assert_eq!(
            unicode_8_digit_escape_sequence("\\U12345678"),
            Ok(("", StringFragment::EscapedUnicode("12345678")))
        );

        assert_errors(unicode_8_digit_escape_sequence("\\U1234567\n"));
        assert_errors(unicode_8_digit_escape_sequence("\\U1234567\r"));
        assert_errors(unicode_8_digit_escape_sequence("\\U1234567"));
        assert_errors(unicode_8_digit_escape_sequence("\\U1234567z"));
        assert_errors(unicode_8_digit_escape_sequence("\\U1234567"));
        assert_errors(unicode_8_digit_escape_sequence("\\u12345678"));
    }

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
        let options = StringOptions::new();
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
                    StringFragment::Escaped("u"),
                    StringFragment::Unescaped("1"),
                    StringFragment::Unescaped("2"),
                    StringFragment::Unescaped("3"),
                    StringFragment::Unescaped("4"),
                ]
            ))
        );
        assert_eq!(
            parse_single_quoted_string(StringOptions::new().allow_unicode_4_digit_escape())(
                "'a\\u1234'"
            ),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::EscapedUnicode("1234"),
                ]
            ))
        );
        assert_eq!(
            parse_single_quoted_string(StringOptions::new().allow_unicode_8_digit_escape())(
                "'a\\u1234'"
            ),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::Escaped("u"),
                    StringFragment::Unescaped("1"),
                    StringFragment::Unescaped("2"),
                    StringFragment::Unescaped("3"),
                    StringFragment::Unescaped("4"),
                ]
            ))
        );
        assert_eq!(
            parse_single_quoted_string(options)("'a\\U12345678'"),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::Escaped("U"),
                    StringFragment::Unescaped("1"),
                    StringFragment::Unescaped("2"),
                    StringFragment::Unescaped("3"),
                    StringFragment::Unescaped("4"),
                    StringFragment::Unescaped("5"),
                    StringFragment::Unescaped("6"),
                    StringFragment::Unescaped("7"),
                    StringFragment::Unescaped("8"),
                ]
            ))
        );
        assert_eq!(
            parse_single_quoted_string(StringOptions::new().allow_unicode_8_digit_escape())(
                "'a\\U12345678'"
            ),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::EscapedUnicode("12345678"),
                ]
            ))
        );
        assert_eq!(
            parse_single_quoted_string(StringOptions::new().allow_unicode_4_digit_escape())(
                "'a\\U12345678'"
            ),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::Escaped("U"),
                    StringFragment::Unescaped("1"),
                    StringFragment::Unescaped("2"),
                    StringFragment::Unescaped("3"),
                    StringFragment::Unescaped("4"),
                    StringFragment::Unescaped("5"),
                    StringFragment::Unescaped("6"),
                    StringFragment::Unescaped("7"),
                    StringFragment::Unescaped("8"),
                ]
            ))
        );
    }

    #[test]
    fn test_parse_double_quoted_string() {
        let options = StringOptions::new();
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
                    StringFragment::Escaped("u"),
                    StringFragment::Unescaped("1"),
                    StringFragment::Unescaped("2"),
                    StringFragment::Unescaped("3"),
                    StringFragment::Unescaped("4"),
                ]
            ))
        );
        assert_eq!(
            parse_double_quoted_string(StringOptions::new().allow_unicode_4_digit_escape())(
                "\"a\\u1234\""
            ),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::EscapedUnicode("1234"),
                ]
            ))
        );
        assert_eq!(
            parse_double_quoted_string(StringOptions::new().allow_unicode_8_digit_escape())(
                "\"a\\u1234\""
            ),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::Escaped("u"),
                    StringFragment::Unescaped("1"),
                    StringFragment::Unescaped("2"),
                    StringFragment::Unescaped("3"),
                    StringFragment::Unescaped("4"),
                ]
            ))
        );
        assert_eq!(
            parse_double_quoted_string(options)("\"a\\U12345678\""),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::Escaped("U"),
                    StringFragment::Unescaped("1"),
                    StringFragment::Unescaped("2"),
                    StringFragment::Unescaped("3"),
                    StringFragment::Unescaped("4"),
                    StringFragment::Unescaped("5"),
                    StringFragment::Unescaped("6"),
                    StringFragment::Unescaped("7"),
                    StringFragment::Unescaped("8"),
                ]
            ))
        );
        assert_eq!(
            parse_double_quoted_string(StringOptions::new().allow_unicode_8_digit_escape())(
                "\"a\\U12345678\""
            ),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::EscapedUnicode("12345678"),
                ]
            ))
        );
        assert_eq!(
            parse_double_quoted_string(StringOptions::new().allow_unicode_4_digit_escape())(
                "\"a\\U12345678\""
            ),
            Ok((
                "",
                vec![
                    StringFragment::Unescaped("a"),
                    StringFragment::Escaped("U"),
                    StringFragment::Unescaped("1"),
                    StringFragment::Unescaped("2"),
                    StringFragment::Unescaped("3"),
                    StringFragment::Unescaped("4"),
                    StringFragment::Unescaped("5"),
                    StringFragment::Unescaped("6"),
                    StringFragment::Unescaped("7"),
                    StringFragment::Unescaped("8"),
                ]
            ))
        );
    }

    #[test]
    fn test_parse_string() {
        let options = StringOptions::new();
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
        let options = StringOptions::new();
        assert_formatted(single_quoted_string(options)("'a'"), ("", "'a'"));
        assert_formatted(single_quoted_string(options)("'\\a'"), ("", "'a'"));
        assert_formatted(
            single_quoted_string(StringOptions::new().backslash_escaped_characters("a"))("'\\a'"),
            ("", "'\\a'"),
        );
        assert_formatted(single_quoted_string(options)("'\\''"), ("", "'\\''"));
    }

    #[test]
    fn test_double_quoted_string() {
        let options = StringOptions::new();
        assert_formatted(double_quoted_string(options)("\"a\""), ("", "\"a\""));
        assert_formatted(double_quoted_string(options)("\"\\a\""), ("", "\"a\""));
        assert_formatted(
            double_quoted_string(StringOptions::new().backslash_escaped_characters("a"))("\"\\a\""),
            ("", "\"\\a\""),
        );
        assert_formatted(double_quoted_string(options)("\"\\\"\""), ("", "\"\\\"\""));
    }

    #[test]
    fn test_parse_and_format_string() {
        let options = StringOptions::new();
        assert_formatted(parse_and_format_string(options)("\"a\""), ("", "\"a\""));
        assert_formatted(parse_and_format_string(options)("'a'"), ("", "\"a\""));
        assert_formatted(parse_and_format_string(options)("'\\a'"), ("", "\"a\""));
        assert_formatted(
            parse_and_format_string(StringOptions::new().backslash_escaped_characters("a"))(
                "'\\a'",
            ),
            ("", "\"\\a\""),
        );
        assert_formatted(parse_and_format_string(options)("'\\''"), ("", "\"'\""));
        assert_formatted(parse_and_format_string(options)("'\\\"'"), ("", "'\"'"));
        assert_formatted(parse_and_format_string(options)("\"\\\"\""), ("", "'\"'"));
        assert_formatted(parse_and_format_string(options)("'\\\\'"), ("", "\"\\\\\""));

        assert_formatted(
            parse_and_format_string(StringOptions::new().preferred_quote_type(QuoteType::Single))(
                "'\\\\'",
            ),
            ("", "'\\\\'"),
        );
        assert_formatted(
            parse_and_format_string(StringOptions::new().preferred_quote_type(QuoteType::Double))(
                "\"\\\"\"",
            ),
            ("", "\"\\\"\""),
        );
    }

    #[test]
    fn format_unicode() {
        assert_formatted(
            parse_and_format_string(
                StringOptions::new()
                    .preferred_quote_type(QuoteType::Single)
                    .allow_unicode_4_digit_escape(),
            )("'\\uBeEf'"),
            ("", "'\\uBEEF'"),
        );
        assert_formatted(
            parse_and_format_string(
                StringOptions::new()
                    .preferred_quote_type(QuoteType::Single)
                    .allow_unicode_4_digit_escape()
                    .unicode_transform_lowercase(),
            )("'\\uBeEf'"),
            ("", "'\\ubeef'"),
        );
        assert_formatted(
            parse_and_format_string(
                StringOptions::new()
                    .preferred_quote_type(QuoteType::Single)
                    .allow_unicode_8_digit_escape(),
            )("'\\UDeAdBeEf'"),
            ("", "'\\UDEADBEEF'"),
        );
        assert_formatted(
            parse_and_format_string(
                StringOptions::new()
                    .preferred_quote_type(QuoteType::Single)
                    .allow_unicode_8_digit_escape()
                    .unicode_transform_lowercase(),
            )("'\\UDeAdBeEf'"),
            ("", "'\\Udeadbeef'"),
        );
    }
}
