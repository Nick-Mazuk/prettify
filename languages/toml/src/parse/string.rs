use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::char,
    combinator::{map, opt, recognize},
    sequence::tuple,
};
use prettify::{string, PrettifyDoc};
use prettify_shared::{double_quoted_string, StringOptions};

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

        assert_errors(single_line_string("'\n'"));
        assert_errors(single_line_string("\"\n\""));
        assert_errors(single_line_string("'\r\n'"));
        assert_errors(single_line_string("\"\r\n\""));
    }
}
