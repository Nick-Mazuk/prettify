use prettify::PrettifyDoc;
use prettify_shared::{parse_and_format_string, QuoteType, StringOptions};

pub fn json_string(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    parse_and_format_string(
        StringOptions::new()
            .backslash_escaped_characters("bfnrt")
            .allow_unicode_4_digit_escape()
            .force_quote_type(QuoteType::Double),
    )(input)
}

#[cfg(test)]
mod test {
    use prettify_shared::assert_formatted;

    use super::*;

    #[test]
    fn test_json_string() {
        assert_formatted(json_string("\"\""), ("", "\"\""));
        assert_formatted(json_string("''"), ("", "\"\""));
        assert_formatted(
            json_string("\"\\b\\f\\n\\r\\t\\\"\\a\""),
            ("", "\"\\b\\f\\n\\r\\t\\\"a\""),
        );
        assert_formatted(json_string("\"\\u1234\""), ("", "\"\\u1234\""));
        assert_formatted(json_string("\"\\U12345678\""), ("", "\"U12345678\""));
        assert_formatted(json_string("\"\\\"\\\"\""), ("", "\"\\\"\\\"\""));
        assert_formatted(json_string("\"\\''\""), ("", "\"''\""));
    }
}
