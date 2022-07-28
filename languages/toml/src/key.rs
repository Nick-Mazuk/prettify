use crate::helpers::is_alphanumeric_or_underscore_or_dash;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    combinator::{map, recognize},
    multi::separated_list1,
    sequence::delimited,
};
use prettify::{join, string, PrettifyDoc};

use super::string::single_line_string;

pub fn key(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, parts) =
        separated_list1(delimiter, alt((single_line_string, bare_key)))(input)?;
    Ok((remainder, join(parts, string("."))))
}

pub fn raw_key(input: &str) -> nom::IResult<&str, Vec<&str>> {
    let (remainder, parts) = separated_list1(
        delimiter,
        alt((recognize(single_line_string), raw_bare_key)),
    )(input)?;
    Ok((remainder, parts))
}

fn delimiter(input: &str) -> nom::IResult<&str, &str> {
    delimited(space0, tag("."), space0)(input)
}

fn raw_bare_key(input: &str) -> nom::IResult<&str, &str> {
    take_while1(is_alphanumeric_or_underscore_or_dash)(input)
}

fn bare_key(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    map(raw_bare_key, string)(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify_shared::assert_formatted;

    #[test]
    fn bare_key_test() {
        assert_formatted(key("foo"), ("", "foo"));
        assert_formatted(key("bare"), ("", "bare"));
        assert_formatted(key("bare-key"), ("", "bare-key"));
        assert_formatted(key("bare_key"), ("", "bare_key"));
        assert_formatted(key("1234"), ("", "1234"));
    }

    #[test]
    fn quoted_key_test() {
        assert_formatted(key("\"127.0.0.1\""), ("", "\"127.0.0.1\""));
        assert_formatted(key("'127.0.0.1'"), ("", "'127.0.0.1'"));
        assert_formatted(
            key("\"character encoding\""),
            ("", "\"character encoding\""),
        );
        assert_formatted(key("\"ʎǝʞ\""), ("", "\"ʎǝʞ\""));
        assert_formatted(key("'quoted \"value\"'"), ("", "'quoted \"value\"'"));
        assert_formatted(key("\"\""), ("", "\"\""));
        assert_formatted(key("''"), ("", "''"));
    }

    #[test]
    fn joined_key_test() {
        assert_formatted(key("name"), ("", "name"));
        assert_formatted(key("physical.color"), ("", "physical.color"));
        assert_formatted(key("site.\"google.com\""), ("", "site.\"google.com\""));
        assert_formatted(
            key("site.\"google.com\".example.co"),
            ("", "site.\"google.com\".example.co"),
        );
        assert_formatted(key("fruit. color"), ("", "fruit.color"));
        assert_formatted(key("fruit . flavor"), ("", "fruit.flavor"));
        assert_formatted(key("fruit     .      flavor"), ("", "fruit.flavor"));
        assert_formatted(key("fruit\t . \t flavor"), ("", "fruit.flavor"));
        assert_formatted(key("3.14159"), ("", "3.14159"));
    }

    #[test]
    fn raw_bare_key_test() {
        assert_eq!(raw_key("foo"), Ok(("", vec!["foo"])));
        assert_eq!(raw_key("bare"), Ok(("", vec!["bare"])));
        assert_eq!(raw_key("bare-key"), Ok(("", vec!["bare-key"])));
        assert_eq!(raw_key("bare_key"), Ok(("", vec!["bare_key"])));
        assert_eq!(raw_key("1234"), Ok(("", vec!["1234"])));
    }

    #[test]
    fn raw_quoted_key_test() {
        assert_eq!(raw_key("\"127.0.0.1\""), Ok(("", vec!["\"127.0.0.1\""])));
        assert_eq!(raw_key("'127.0.0.1'"), Ok(("", vec!["'127.0.0.1'"])));
        assert_eq!(
            raw_key("\"character encoding\""),
            Ok(("", vec!["\"character encoding\""]))
        );
        assert_eq!(raw_key("\"ʎǝʞ\""), Ok(("", vec!["\"ʎǝʞ\""])));
        assert_eq!(
            raw_key("'quoted \"value\"'"),
            Ok(("", vec!["'quoted \"value\"'"]))
        );
        assert_eq!(raw_key("\"\""), Ok(("", vec!["\"\""])));
        assert_eq!(raw_key("''"), Ok(("", vec!["''"])));
    }

    #[test]
    fn raw_joined_key_test() {
        assert_eq!(raw_key("name"), Ok(("", vec!["name"])));
        assert_eq!(
            raw_key("physical.color"),
            Ok(("", vec!["physical", "color"]))
        );
        assert_eq!(
            raw_key("site.\"google.com\""),
            Ok(("", vec!["site", "\"google.com\""]))
        );
        assert_eq!(
            raw_key("site.\"google.com\".example.co"),
            Ok(("", vec!["site", "\"google.com\"", "example", "co"]))
        );
        assert_eq!(raw_key("fruit. color"), Ok(("", vec!["fruit", "color"])));
        assert_eq!(raw_key("fruit . flavor"), Ok(("", vec!["fruit", "flavor"])));
        assert_eq!(
            raw_key("fruit     .      flavor"),
            Ok(("", vec!["fruit", "flavor"]))
        );
        assert_eq!(
            raw_key("fruit\t . \t flavor"),
            Ok(("", vec!["fruit", "flavor"]))
        );
        assert_eq!(raw_key("3.14159"), Ok(("", vec!["3", "14159"])));
    }
}
