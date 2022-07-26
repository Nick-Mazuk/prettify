use crate::helpers::is_alphanumeric_or_underscore_or_dash;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::space0,
    combinator::map,
    multi::separated_list1,
    sequence::delimited,
};
use prettify::{join, string, PrettifyDoc};

use super::string::single_line_string;

pub fn key(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, parts) = separated_list1(
        delimited(space0, tag("."), space0),
        alt((single_line_string, bare_key)),
    )(input)?;
    Ok((remainder, join(parts, string("."))))
}

fn bare_key(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    map(take_while1(is_alphanumeric_or_underscore_or_dash), string)(input)
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
}
