use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_till1},
    combinator::{eof, recognize, rest},
    multi::{many0, many1},
    sequence::terminated,
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

pub fn line_ending(input: &str) -> nom::IResult<&str, &str> {
    alt((tag(NEWLINE_STR), tag(CARRIAGE_RETURN_STR), tag("\r\n"), eof))(input)
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
    take_till(is_whitespace_char)(input)
}

pub fn whitespace1(input: &str) -> nom::IResult<&str, &str> {
    take_till1(is_whitespace_char)(input)
}

pub fn is_space(char: char) -> bool {
    char == SPACE_CHAR
}

pub fn space(input: &str) -> nom::IResult<&str, &str> {
    tag(SPACE_STR)(input)
}

pub fn space0(input: &str) -> nom::IResult<&str, &str> {
    recognize(many0(tag(SPACE_STR)))(input)
}

pub fn space1(input: &str) -> nom::IResult<&str, &str> {
    recognize(many1(tag(SPACE_STR)))(input)
}
