use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while, take_while1},
    character::complete::anychar,
    combinator::{eof, peek, recognize, rest},
    multi::many_till,
    sequence::{terminated, tuple},
};

pub const NEWLINE_CHAR: char = '\n';
pub const NEWLINE_STR: &str = "\n";
pub const CARRIAGE_RETURN_CHAR: char = '\r';
pub const CARRIAGE_RETURN_STR: &str = "\r";
pub const SPACE_CHAR: char = ' ';
pub const SPACE_STR: &str = " ";
pub const TAB_CHAR: char = '\t';
pub const TAB_STR: &str = "\t";
pub const LINE_TABULATION_CHAR: char = '\x0B';
pub const LINE_TABULATION_STR: &str = "\x0B";
pub const FORM_FEED_CHAR: char = '\x0C';
pub const FORM_FEED_STR: &str = "\x0C";

pub fn line_ending_no_eof(input: &str) -> nom::IResult<&str, &str> {
    alt((tag(NEWLINE_STR), tag(CARRIAGE_RETURN_STR), tag("\r\n")))(input)
}

pub fn line_ending(input: &str) -> nom::IResult<&str, &str> {
    alt((line_ending_no_eof, eof))(input)
}

pub fn line(input: &str) -> nom::IResult<&str, &str> {
    terminated(
        alt((
            take_till(|char| char == NEWLINE_CHAR || char == CARRIAGE_RETURN_CHAR),
            rest,
        )),
        line_ending,
    )(input)
}

pub fn is_whitespace_char(char: char) -> bool {
    char == SPACE_CHAR
        || char == TAB_CHAR
        || char == NEWLINE_CHAR
        || char == CARRIAGE_RETURN_CHAR
        || char == LINE_TABULATION_CHAR
        || char == FORM_FEED_CHAR
}

pub fn whitespace0(input: &str) -> nom::IResult<&str, &str> {
    take_while(is_whitespace_char)(input)
}

pub fn whitespace1(input: &str) -> nom::IResult<&str, &str> {
    take_while1(is_whitespace_char)(input)
}

pub fn is_space(char: char) -> bool {
    char == SPACE_CHAR
}

pub fn space(input: &str) -> nom::IResult<&str, &str> {
    tag(SPACE_STR)(input)
}

pub fn space0(input: &str) -> nom::IResult<&str, &str> {
    take_while(is_space)(input)
}

pub fn space1(input: &str) -> nom::IResult<&str, &str> {
    take_while1(is_space)(input)
}

pub fn is_inline_whitespace(char: char) -> bool {
    char == SPACE_CHAR || char == TAB_CHAR
}

pub fn inline_whitespace0(input: &str) -> nom::IResult<&str, &str> {
    take_while(is_inline_whitespace)(input)
}

pub fn inline_whitespace1(input: &str) -> nom::IResult<&str, &str> {
    take_while1(is_inline_whitespace)(input)
}

pub fn any_until_line_ending(input: &str) -> nom::IResult<&str, &str> {
    recognize(many_till(anychar, peek(line_ending)))(input)
}

pub fn block_ending(input: &str) -> nom::IResult<&str, &str> {
    alt((
        recognize(tuple((line_ending_no_eof, line_ending_no_eof))),
        recognize(tuple((line_ending_no_eof, eof))),
        eof,
    ))(input)
}

pub fn any_until_block_ending(input: &str) -> nom::IResult<&str, &str> {
    recognize(many_till(anychar, peek(block_ending)))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_ending_test() {
        assert_eq!(line_ending("\n"), Ok(("", "\n")));
        assert_eq!(line_ending("\n\n"), Ok(("\n", "\n")));
        assert_eq!(line_ending("\n\nhello"), Ok(("\nhello", "\n")));
        assert_eq!(line_ending("\n"), Ok(("", "\n")));
        assert_eq!(line_ending("\nhello"), Ok(("hello", "\n")));

        // only matches a 0 line breaks if it's the end of the file
        assert_eq!(line_ending(""), Ok(("", "")));
        assert_eq!(
            line_ending("hello"),
            Err(nom::Err::Error(nom::error::Error {
                input: "hello",
                code: nom::error::ErrorKind::Eof,
            }))
        );
    }
}
