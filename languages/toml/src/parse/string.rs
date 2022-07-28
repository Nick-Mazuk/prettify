use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::char,
    combinator::{map, opt, recognize},
    sequence::tuple,
};
use prettify::{string, PrettifyDoc};
use prettify_shared::{custom_quoted_string, double_quoted_string, StringOptions};

pub fn single_line_string(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    alt((
        double_quoted_string(StringOptions {
            backslash_escaped_characters: "btnfr",
            allow_line_breaks: false,
        }),
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
        custom_quoted_string(
            "\"\"\"",
            StringOptions {
                backslash_escaped_characters: "btnfr\"",
                allow_line_breaks: true,
            },
        ),
        map(
            recognize(tuple((tag("'''"), take_until("'''"), tag("'''")))),
            string,
        ),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify_shared::{assert_errors, assert_formatted};

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
        assert_formatted(
            multi_line_string("\"\"\"\nRoses are red\nViolets are blue\"\"\""),
            ("", "\"\"\"\nRoses are red\nViolets are blue\"\"\""),
        );
        assert_formatted(
            multi_line_string("'''\nRoses are red\nViolets are blue'''"),
            ("", "'''\nRoses are red\nViolets are blue'''"),
        );
    }
}
