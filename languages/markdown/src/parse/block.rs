use super::{header::header, paragraph::paragraph};
use crate::nodes::Block;
use nom::{
    branch::alt,
    character::complete::line_ending,
    combinator::{eof, recognize},
    multi::many0,
    sequence::{terminated, tuple},
};

pub fn parse_blocks(input: &str) -> nom::IResult<&str, Vec<Block>> {
    many0(block)(input)
}

pub fn block(input: &str) -> nom::IResult<&str, Block> {
    terminated(alt((header, paragraph)), block_end)(input)
}

pub fn block_end(input: &str) -> nom::IResult<&str, &str> {
    let result = alt((
        recognize(tuple((line_ending, line_ending, many0(line_ending)))),
        recognize(tuple((line_ending, eof))),
        eof,
    ))(input);
    match result {
        Ok((remainder, breaks)) => Ok((remainder, breaks)),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::nodes::Leaf;

    #[test]
    fn block_end_test() {
        assert_eq!(block_end("\n\n"), Ok(("", "\n\n")));
        assert_eq!(block_end("\n\nhello"), Ok(("hello", "\n\n")));
        assert_eq!(block_end("\n\n\n"), Ok(("", "\n\n\n")));
        assert_eq!(block_end("\n\n\nhello"), Ok(("hello", "\n\n\n")));

        // only matches a single line break if it's the end of the file
        assert_eq!(block_end("\n"), Ok(("", "\n")));
        assert_eq!(
            block_end("\nhello"),
            Err(nom::Err::Error(nom::error::Error {
                input: "\nhello",
                code: nom::error::ErrorKind::Eof,
            }))
        );

        // only matches a 0 line breaks if it's the end of the file
        assert_eq!(block_end(""), Ok(("", "")));
        assert_eq!(
            block_end("hello"),
            Err(nom::Err::Error(nom::error::Error {
                input: "hello",
                code: nom::error::ErrorKind::Eof,
            }))
        );
    }

    #[test]
    fn block_header() {
        assert_eq!(
            block("# hello world"),
            Ok((
                "",
                Block::Header(1, vec![Leaf::String("hello world".to_string())])
            ))
        );
        assert_eq!(
            block("# hello world\n\n"),
            Ok((
                "",
                Block::Header(1, vec![Leaf::String("hello world".to_string())])
            ))
        );
    }
}
