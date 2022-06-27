use super::{empty_line::empty_line, header::header, paragraph::paragraph};
use crate::nodes::Block;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::line_ending,
    combinator::{eof, opt, peek, recognize},
    multi::many0,
    sequence::terminated,
};

pub fn parse_blocks(input: &str) -> nom::IResult<&str, Vec<Block>> {
    many0(block)(input)
}

pub fn block(input: &str) -> nom::IResult<&str, Block> {
    alt((empty_line, header, paragraph))(input)
}

pub fn block_end(input: &str) -> nom::IResult<&str, &str> {
    let result = opt(alt((tag("\n"), eof)))(input);
    match result {
        Ok((remainder, char)) => match char {
            Some(_) => Ok((remainder, "")),
            _ => Err(nom::Err::Error(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Eof,
            })),
        },
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::nodes::Leaf;

    #[test]
    fn block_end_test() {
        assert_eq!(block_end("\n\n"), Ok(("\n", "")));
        assert_eq!(block_end("\n\nhello"), Ok(("\nhello", "")));
        assert_eq!(block_end("\n"), Ok(("", "")));
        assert_eq!(block_end("\nhello"), Ok(("hello", "")));

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

    // #[test]
    // fn block_header() {
    //     assert_eq!(
    //         block("# hello world"),
    //         Ok((
    //             "",
    //             Block::Header(1, vec![Leaf::String("hello world".to_string())])
    //         ))
    //     );
    //     assert_eq!(
    //         block("## hello world\n\n"),
    //         Ok((
    //             "\n",
    //             Block::Header(2, vec![Leaf::String("hello world".to_string())])
    //         ))
    //     );
    // }

    // #[test]
    // fn block_empty_line() {
    //     assert_eq!(block(""), Ok(("", Block::EmptyLine)));
    //     assert_eq!(block("\n"), Ok(("", Block::EmptyLine)));
    //     assert_eq!(block("\n\n"), Ok(("\n", Block::EmptyLine)));
    // }
}
