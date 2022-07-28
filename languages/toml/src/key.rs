use super::string::single_line_string;
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

#[derive(PartialEq, Debug, Clone)]
pub struct KeyValuePair<'a> {
    pub prettify_doc: PrettifyDoc<'a>,
    pub raw_key: Vec<&'a str>,
}

pub fn key(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, parts) =
        separated_list1(delimiter, alt((single_line_string, bare_key)))(input)?;
    Ok((remainder, join(parts, string("."))))
}

pub fn raw_key(input: &str) -> nom::IResult<&str, KeyValuePair> {
    let (remainder, parts) = separated_list1(
        delimiter,
        alt((recognize(single_line_string), raw_bare_key)),
    )(input)?;
    Ok((
        remainder,
        KeyValuePair {
            prettify_doc: key(input)?.1,
            raw_key: parts,
        },
    ))
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
        let parsed_key = raw_key("foo").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["foo"]);

        let parsed_key = raw_key("bare").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["bare"]);

        let parsed_key = raw_key("bare-key").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["bare-key"]);

        let parsed_key = raw_key("bare_key").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["bare_key"]);

        let parsed_key = raw_key("1234").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["1234"]);
    }

    #[test]
    fn raw_quoted_key_test() {
        let parsed_key = raw_key("\"127.0.0.1\"").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["\"127.0.0.1\""]);

        let parsed_key = raw_key("'127.0.0.1'").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["'127.0.0.1'"]);

        let parsed_key = raw_key("\"character encoding\"").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["\"character encoding\""]);

        let parsed_key = raw_key("\"ʎǝʞ\"").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["\"ʎǝʞ\""]);

        let parsed_key = raw_key("'quoted \"value\"'").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["'quoted \"value\"'"]);

        let parsed_key = raw_key("\"\"").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["\"\""]);

        let parsed_key = raw_key("''").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["''"]);
    }

    #[test]
    fn raw_joined_key_test() {
        let parsed_key = raw_key("name").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["name"]);

        let parsed_key = raw_key("physical.color").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["physical", "color"]);

        let parsed_key = raw_key("site.\"google.com\"").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["site", "\"google.com\""]);

        let parsed_key = raw_key("site.\"google.com\".example.co").unwrap().1;
        assert_eq!(
            parsed_key.raw_key,
            vec!["site", "\"google.com\"", "example", "co"]
        );

        let parsed_key = raw_key("fruit. color").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["fruit", "color"]);

        let parsed_key = raw_key("fruit . flavor").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["fruit", "flavor"]);

        let parsed_key = raw_key("fruit     .      flavor").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["fruit", "flavor"]);

        let parsed_key = raw_key("fruit\t . \t flavor").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["fruit", "flavor"]);

        let parsed_key = raw_key("3.14159").unwrap().1;
        assert_eq!(parsed_key.raw_key, vec!["3", "14159"]);
    }
}
