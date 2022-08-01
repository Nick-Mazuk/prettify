use crate::{string::json_string, value::value};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::multispace0,
    combinator::map,
    sequence::{separated_pair, tuple},
};
use prettify::{concat, string, PrettifyDoc};
use prettify_shared::{repeated_items, RepeatedItemsOptions};

pub fn object(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    repeated_items(
        RepeatedItemsOptions::new("{", key_value_pair, ",", "}").use_user_preferred_indentation(),
    )(input)
}

fn key(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    alt((
        json_string,
        map(is_not(":\n\r"), |result: &str| {
            concat(vec![string("\""), string(result.trim()), string("\"")])
        }),
    ))(input)
}

fn key_value_pair(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    map(
        separated_pair(key, tuple((multispace0, tag(":"), multispace0)), value),
        |(key, value)| concat(vec![key, string(": "), value]),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify_shared::{assert_errors, assert_formatted};

    #[test]
    fn test_object() {
        assert_formatted(
            object("{\"hello\": \"world\"}"),
            ("", "{\"hello\": \"world\"}"),
        );
        assert_formatted(
            object("{'hello': \"world\"}"),
            ("", "{\"hello\": \"world\"}"),
        );
        assert_formatted(object("{hello: \"world\"}"), ("", "{\"hello\": \"world\"}"));
        assert_formatted(object("{\"\": \"world\"}"), ("", "{\"\": \"world\"}"));
        assert_formatted(object("{'': \"world\"}"), ("", "{\"\": \"world\"}"));
        assert_formatted(
            object("{\n\"hello\"\n :\n\"world\"}"),
            ("", "{\n    \"hello\": \"world\"\n}"),
        );
        assert_formatted(
            object("{  hello world  : 42}"),
            ("", "{\"hello world\": 42}"),
        );
        assert_formatted(
            object("{\"this is a long key\": 1234,\"this is a long key\": 1234,\"this is a long key\": 1234}"),
            ("", "{\n    \"this is a long key\": 1_234,\n    \"this is a long key\": 1_234,\n    \"this is a long key\": 1_234\n}"),
        );
        assert_formatted(
            object("{\"hello\": {\"world\": 42}}"),
            ("", "{\"hello\": {\"world\": 42}}"),
        );
        assert_formatted(
            object("{\"hello 1\": [\"world\", 42], \"hello 2\": {\"world\": 42}}"),
            (
                "",
                "{\"hello 1\": [\"world\", 42], \"hello 2\": {\"world\": 42}}",
            ),
        );
        assert_formatted(
            object("{\n\"hello 1\": [\"world\", 42], \"hello 2\": {\"world\": 42}}"),
            (
                "",
                "{\n    \"hello 1\": [\"world\", 42],\n    \"hello 2\": {\"world\": 42}\n}",
            ),
        );
        assert_formatted(
            object("{\"hello 1\": [\"world\", 42], \"hello 2\": {\n\"world\": 42}}"),
            (
                "",
                "{\n    \"hello 1\": [\"world\", 42],\n    \"hello 2\": {\n        \"world\": 42\n    }\n}",
            ),
        );

        assert_errors(object("{: \"world\"}"));
    }
}
