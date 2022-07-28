use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{map, recognize},
    sequence::{preceded, tuple},
};
use prettify::{concat, string, PrettifyDoc};

use crate::{
    key::{key, raw_key},
    line_endings::line_end_with_optional_comment,
};

#[derive(PartialEq, Debug, Clone)]
pub struct TableHeader<'a> {
    pub key: PrettifyDoc<'a>,
    pub raw_key: Vec<&'a str>,
    pub repeated: bool,
}

pub fn table_header(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    map(raw_table_header, |result| result.key)(input)
}

pub fn raw_table_header(input: &str) -> nom::IResult<&str, TableHeader> {
    alt((raw_repeated_table_header, raw_unrepeated_table_header))(input)
}

fn opening_unrepeated_delimiter(input: &str) -> nom::IResult<&str, &str> {
    recognize(tuple((space0, tag("["), space0)))(input)
}

fn closing_unrepeated_delimiter_and_comment(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    preceded(
        tuple((space0, tag("]"), space0)),
        line_end_with_optional_comment,
    )(input)
}

fn parsed_unrepeated_table_header(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, (_, parsed_key, comment)) = tuple((
        opening_unrepeated_delimiter,
        key,
        closing_unrepeated_delimiter_and_comment,
    ))(input)?;
    Ok((
        remainder,
        concat(vec![string("["), parsed_key, string("]"), comment]),
    ))
}

fn raw_unrepeated_table_header(input: &str) -> nom::IResult<&str, TableHeader> {
    let (remainder, (_, raw_key, _)) = tuple((
        opening_unrepeated_delimiter,
        raw_key,
        closing_unrepeated_delimiter_and_comment,
    ))(input)?;
    Ok((
        remainder,
        TableHeader {
            key: parsed_unrepeated_table_header(input).unwrap().1,
            raw_key,
            repeated: false,
        },
    ))
}

fn opening_repeated_delimiter(input: &str) -> nom::IResult<&str, &str> {
    recognize(tuple((space0, tag("[["), space0)))(input)
}

fn closing_repeated_delimiter_and_comment(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    preceded(
        tuple((space0, tag("]]"), space0)),
        line_end_with_optional_comment,
    )(input)
}

fn parsed_repeated_table_header(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, (_, parsed_key, comment)) = tuple((
        opening_repeated_delimiter,
        key,
        closing_repeated_delimiter_and_comment,
    ))(input)?;
    Ok((
        remainder,
        concat(vec![string("[["), parsed_key, string("]]"), comment]),
    ))
}

fn raw_repeated_table_header(input: &str) -> nom::IResult<&str, TableHeader> {
    let (remainder, (_, raw_key, _)) = tuple((
        opening_repeated_delimiter,
        raw_key,
        closing_repeated_delimiter_and_comment,
    ))(input)?;
    Ok((
        remainder,
        TableHeader {
            key: parsed_repeated_table_header(input).unwrap().1,
            raw_key,
            repeated: true,
        },
    ))
}

#[cfg(test)]
mod test {
    use prettify_shared::assert_formatted;

    use super::*;

    #[test]
    fn raw_table_header_test() {
        let header = raw_table_header("[foo]").unwrap().1;
        assert_eq!(header.raw_key, vec!["foo"]);
        assert_eq!(header.repeated, false);

        let header = raw_table_header("[[foo]]").unwrap().1;
        assert_eq!(header.raw_key, vec!["foo"]);
        assert_eq!(header.repeated, true);

        let header = raw_table_header("[foo]\n").unwrap().1;
        assert_eq!(header.raw_key, vec!["foo"]);
        assert_eq!(header.repeated, false);

        let header = raw_table_header("[[foo]]\n").unwrap().1;
        assert_eq!(header.raw_key, vec!["foo"]);
        assert_eq!(header.repeated, true);

        let header = raw_table_header("[foo.bar]").unwrap().1;
        assert_eq!(header.raw_key, vec!["foo", "bar"]);
        assert_eq!(header.repeated, false);

        let header = raw_table_header("[[foo.bar]]").unwrap().1;
        assert_eq!(header.raw_key, vec!["foo", "bar"]);
        assert_eq!(header.repeated, true);

        let header = raw_table_header("[foo.\"bar.fizz.buzz\"]").unwrap().1;
        assert_eq!(header.raw_key, vec!["foo", "\"bar.fizz.buzz\""]);
        assert_eq!(header.repeated, false);
    }

    #[test]
    fn table_header_test() {
        assert_formatted(
            table_header(" \t[[   hello.world ]]#this is a comment\n"),
            ("", "[[hello.world]] # this is a comment\n"),
        );

        assert_formatted(
            table_header(" \t[   hello. \"world\" ]     #    this is a comment\n"),
            ("", "[hello.\"world\"] # this is a comment\n"),
        );
    }
}
