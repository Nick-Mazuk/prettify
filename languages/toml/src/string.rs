use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take},
    character::complete::char,
    combinator::not,
    combinator::{map, opt, peek, recognize},
    multi::many_till,
    sequence::{delimited, tuple},
};
use prettify::{string, PrettifyDoc};
use prettify_shared::{
    backslash_escape, double_quoted_string, format_custom_quoted_string, unescaped_char_multiline,
    unicode_escape_sequence, StringOptions,
};

pub fn toml_string(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    alt((multi_line_string, single_line_string))(input)
}

pub fn single_line_string(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    alt((
        double_quoted_string(
            StringOptions::new()
                .escaped_chars("btnfr")
                .allow_unicode_4_digit_escape()
                .allow_unicode_8_digit_escape(),
        ),
        map(
            recognize(tuple((
                char('\''),
                recognize(opt(is_not("'\n\r"))),
                char('\''),
            ))),
            string,
        ),
    ))(input)
}

pub fn multi_line_string(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    alt((
        map(
            delimited(
                tag("\"\"\""),
                map(
                    many_till(
                        alt((
                            unicode_escape_sequence,
                            backslash_escape,
                            unescaped_char_multiline,
                        )),
                        peek(tuple((tag("\"\"\""), not(tag("\""))))),
                    ),
                    |result| result.0,
                ),
                tuple((tag("\"\"\""), peek(not(tag("\""))))),
            ),
            |result| {
                format_custom_quoted_string(
                    "\"\"\"",
                    result,
                    StringOptions::new()
                        .escaped_chars("btnfr\"")
                        .allow_unicode_4_digit_escape()
                        .allow_unicode_8_digit_escape()
                        .allow_line_breaks(),
                )
            },
        ),
        map(
            recognize(tuple((
                tag("'''"),
                many_till(take(1 as usize), tuple((tag("'''"), not(tag("'"))))),
            ))),
            string,
        ),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify_shared::{assert_errors, assert_formatted};

    #[test]
    fn test_toml_string() {
        assert_formatted(toml_string("\"\""), ("", "\"\""));
        assert_formatted(toml_string("''"), ("", "''"));
        assert_formatted(toml_string("\"\"\"\"\"\""), ("", "\"\"\"\"\"\""));
        assert_formatted(toml_string("''''''"), ("", "''''''"));
    }

    #[test]
    fn single_line_string_test() {
        assert_formatted(single_line_string("\"\""), ("", "\"\""));
        assert_formatted(single_line_string("''"), ("", "''"));
        assert_formatted(single_line_string("\"foo\""), ("", "\"foo\""));
        assert_formatted(single_line_string("'foo'"), ("", "'foo'"));
        assert_formatted(single_line_string("\"foo\\'\""), ("", "\"foo'\""));
        assert_formatted(single_line_string("\"foo\\b\""), ("", "\"foo\\b\""));
        assert_formatted(single_line_string("\"foo\\t\""), ("", "\"foo\\t\""));
        assert_formatted(single_line_string("\"foo\\n\""), ("", "\"foo\\n\""));
        assert_formatted(single_line_string("\"foo\\f\""), ("", "\"foo\\f\""));
        assert_formatted(single_line_string("\"foo\\r\""), ("", "\"foo\\r\""));
        assert_formatted(single_line_string("'foo\\''"), ("'", "'foo\\'"));
        assert_formatted(
            single_line_string("'C:\\Users\\nodejs\\templates'"),
            ("", "'C:\\Users\\nodejs\\templates'"),
        );
        assert_formatted(
            single_line_string("'\\\\ServerX\\admin$\\system32\\'"),
            ("", "'\\\\ServerX\\admin$\\system32\\'"),
        );
        assert_formatted(
            single_line_string("'Tom \"Dubs\" Preston-Werner'"),
            ("", "'Tom \"Dubs\" Preston-Werner'"),
        );
        assert_formatted(
            single_line_string("'<\\i\\c*\\s*>'"),
            ("", "'<\\i\\c*\\s*>'"),
        );
        assert_formatted(
            single_line_string(
                "\"I'm a string. \\\"You can quote me\\\". Name\\tJos\\u00E9\\nLocation\\tSF.\"",
            ),
            (
                "",
                "\"I'm a string. \\\"You can quote me\\\". Name\\tJos\\u00E9\\nLocation\\tSF.\"",
            ),
        );

        assert_errors(single_line_string("'\n'"));
        assert_errors(single_line_string("\"\n\""));
        assert_errors(single_line_string("'\r\n'"));
        assert_errors(single_line_string("\"\r\n\""));
    }

    #[test]
    fn multi_line_string_test() {
        assert_formatted(multi_line_string("\"\"\"\"\"\""), ("", "\"\"\"\"\"\""));
        assert_formatted(multi_line_string("''''''"), ("", "''''''"));
        assert_formatted(
            multi_line_string("'''\nRoses are red\nViolets are blue'''"),
            ("", "'''\nRoses are red\nViolets are blue'''"),
        );
        assert_formatted(
            multi_line_string("\"\"\"\nRoses are red\nViolets are blue\"\"\""),
            ("", "\"\"\"\nRoses are red\nViolets are blue\"\"\""),
        );
        assert_formatted(
            multi_line_string("\"\"\"\"Roses are red\nViolets are blue\"\"\"\""),
            ("", "\"\"\"\"Roses are red\nViolets are blue\"\"\"\""),
        );

        assert_formatted(
            multi_line_string("\"\"\"Here are two quotation marks: \"\". Simple enough.\"\"\""),
            (
                "",
                "\"\"\"Here are two quotation marks: \"\". Simple enough.\"\"\"",
            ),
        );
        assert_formatted(
            multi_line_string("\"\"\"Here are three quotation marks: \"\"\\\".\"\"\""),
            ("", "\"\"\"Here are three quotation marks: \"\"\\\".\"\"\""),
        );
        assert_formatted(
            multi_line_string("\"\"\"Here are fifteen quotation marks: \"\"\\\"\"\"\\\"\"\"\\\"\"\"\\\"\"\"\\\".\"\"\""),
            ("", "\"\"\"Here are fifteen quotation marks: \"\"\\\"\"\"\\\"\"\"\\\"\"\"\\\"\"\"\\\".\"\"\""),
        );
        assert_formatted(
            multi_line_string("\"\"\"\"This,\" she said, \"is just a pointless statement.\"\"\"\""),
            (
                "",
                "\"\"\"\"This,\" she said, \"is just a pointless statement.\"\"\"\"",
            ),
        );
        assert_formatted(
            multi_line_string("'''I [dw]on't need \\d{2} apples'''"),
            ("", "'''I [dw]on't need \\d{2} apples'''"),
        );
        assert_formatted(
            multi_line_string("'''\nThe first newline is\ntrimmed in raw strings.\n   All other whitespace\n   is preserved.\n'''"),
            ("", "'''\nThe first newline is\ntrimmed in raw strings.\n   All other whitespace\n   is preserved.\n'''"),
        );
        assert_formatted(
            multi_line_string(
                "'''Here are fifteen quotation marks: \"\"\"\"\"\"\"\"\"\"\"\"\"\"\"'''",
            ),
            (
                "",
                "'''Here are fifteen quotation marks: \"\"\"\"\"\"\"\"\"\"\"\"\"\"\"'''",
            ),
        );
        assert_formatted(
            multi_line_string("''''That,' she said, 'is still pointless.''''"),
            ("", "''''That,' she said, 'is still pointless.''''"),
        );
    }
}
