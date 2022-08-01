use crate::{
    boolean::boolean, key::key, line_endings::line_end_with_optional_comment, string::toml_string,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    sequence::{separated_pair, tuple},
};
use prettify::{concat, string, PrettifyDoc};
use prettify_shared::{
    float, integer, rfc_3339_date, rfc_3339_date_time, rfc_3339_local_date_time,
    rfc_3339_partial_time, IntegerOptions,
};

#[derive(PartialEq, Debug, Clone)]
pub struct KeyValuePair<'a> {
    pub raw_key: Vec<&'a str>,
    pub prettify_doc: PrettifyDoc<'a>,
}

pub fn value(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    alt((
        boolean,
        rfc_3339_date_time,
        rfc_3339_local_date_time,
        rfc_3339_date,
        rfc_3339_partial_time,
        float,
        integer(
            IntegerOptions::new()
                .use_underscores()
                .parse_binary()
                .parse_hex()
                .parse_octal(),
        ),
        toml_string,
    ))(input)
}

pub fn key_value_pair(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (input, (_, (key, value), end)) = tuple((
        space0,
        separated_pair(key, tuple((space0, tag("="), space0)), value),
        line_end_with_optional_comment,
    ))(input)?;
    Ok((input, concat(vec![key, string(" = "), value, end])))
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify_shared::assert_formatted;

    #[test]
    fn test_boolean_value() {
        assert_formatted(key_value_pair("hello = true"), ("", "hello = true\n"));
        assert_formatted(key_value_pair("   hello=false   "), ("", "hello = false\n"));
    }

    #[test]
    fn test_float_value() {
        assert_formatted(key_value_pair("hello = 1.0"), ("", "hello = 1.0\n"));
        assert_formatted(key_value_pair("   hello=1.0   "), ("", "hello = 1.0\n"));
    }

    #[test]
    fn test_integer_value() {
        assert_formatted(key_value_pair("hello =+123"), ("", "hello = 123\n"));
        assert_formatted(
            key_value_pair("   world= -454356   "),
            ("", "world = -454_356\n"),
        );
    }

    #[test]
    fn test_offset_date_time() {
        assert_formatted(
            key_value_pair("hello = 1979-05-27T07:32:00Z"),
            ("", "hello = 1979-05-27T07:32:00Z\n"),
        );
        assert_formatted(
            key_value_pair("hello = 1979-05-27 07:32:00Z"),
            ("", "hello = 1979-05-27T07:32:00Z\n"),
        );
        assert_formatted(
            key_value_pair("hello = 1979-05-27T00:32:00-07:00"),
            ("", "hello = 1979-05-27T00:32:00-07:00\n"),
        );
        assert_formatted(
            key_value_pair("hello = 1979-05-27T00:32:00.999999-07:00"),
            ("", "hello = 1979-05-27T00:32:00.999999-07:00\n"),
        );
    }

    #[test]
    fn test_local_date_time() {
        assert_formatted(
            key_value_pair("hello = 1979-05-27T07:32:00"),
            ("", "hello = 1979-05-27T07:32:00\n"),
        );
        assert_formatted(
            key_value_pair("hello = 1979-05-27T00:32:00.999999"),
            ("", "hello = 1979-05-27T00:32:00.999999\n"),
        );
    }

    #[test]
    fn test_local_date() {
        assert_formatted(
            key_value_pair("hello = 1979-05-27"),
            ("", "hello = 1979-05-27\n"),
        );
    }

    #[test]
    fn test_local_time() {
        assert_formatted(
            key_value_pair("hello = 07:32:00"),
            ("", "hello = 07:32:00\n"),
        );
        assert_formatted(
            key_value_pair("hello = 00:32:00.999999"),
            ("", "hello = 00:32:00.999999\n"),
        );
    }

    #[test]
    fn test_with_comment() {
        assert_formatted(
            key_value_pair("hello = true#world"),
            ("", "hello = true # world\n"),
        );
    }
}
