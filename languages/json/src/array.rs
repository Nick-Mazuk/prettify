use crate::value::value;
use prettify::PrettifyDoc;
use prettify_shared::{repeated_items, RepeatedItemsOptions};

pub fn array(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    repeated_items(RepeatedItemsOptions::new("[", value, ",", "]").use_user_preferred_indentation())(
        input,
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify_shared::assert_formatted;

    #[test]
    fn test_array() {
        assert_formatted(
            array("[\"hello\", \"world\"]"),
            ("", "[\"hello\", \"world\"]"),
        );
        assert_formatted(
            array("[\n\"hello\", \"world\"]"),
            ("", "[\n    \"hello\",\n    \"world\"\n]"),
        );
        assert_formatted(array("[\n123, 124]"), ("", "[\n    123,\n    124\n]"));
        assert_formatted(
            array("[123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123, 123]"),
            ("", "[\n    123,\n    123,\n    123,\n    123,\n    123,\n    123,\n    123,\n    123,\n    123,\n    123,\n    123,\n    123,\n    123,\n    123,\n    123,\n    123,\n    123\n]"),
        );
        assert_formatted(
            array("[[\"hello\"], [\"world\"]]"),
            ("", "[[\"hello\"], [\"world\"]]"),
        );
        assert_formatted(
            array("[\n[\"hello\"], [\"world\"]]"),
            ("", "[\n    [\"hello\"],\n    [\"world\"]\n]"),
        );
        assert_formatted(
            array("[[\n\"hello\"], [\"world\"]]"),
            (
                "",
                "[\n    [\n        \"hello\"\n    ],\n    [\"world\"]\n]",
            ),
        );
        assert_formatted(array("['',]"), ("", "[\"\"]"));
        assert_formatted(array("[null,]"), ("", "[null]"));
        assert_formatted(array("[false,]"), ("", "[false]"));
    }
}
