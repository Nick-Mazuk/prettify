use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    combinator::recognize,
    multi::{many0, many_m_n},
    sequence::{delimited, terminated, tuple},
};

use crate::{
    nodes::LeafBlock,
    parse::preliminaries::{any_until_line_ending, line_ending, line_ending_no_eof, SPACE_STR},
};

fn code_block_line(input: &str) -> nom::IResult<&str, &str> {
    delimited(
        many_m_n(4, 4, tag(SPACE_STR)),
        recognize(any_until_line_ending),
        line_ending,
    )(input)
}

fn empty_line(input: &str) -> nom::IResult<&str, &str> {
    terminated(is_a(" \t"), line_ending)(input)
}

pub fn indented_code_block(input: &str) -> nom::IResult<&str, LeafBlock> {
    let result = tuple((
        code_block_line,
        many0(alt((
            code_block_line,
            empty_line,
            terminated(tag(""), line_ending_no_eof),
        ))),
    ))(input);

    match result {
        Ok((remainder, (first_line, mut lines))) => {
            lines.insert(0, first_line);
            Ok((remainder, LeafBlock::IndentedCodeBlock(lines)))
        }
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    #[test]
    fn indented_code_block_test() {
        assert_eq!(
            indented_code_block("    foo"),
            Ok(("", LeafBlock::IndentedCodeBlock(vec!["foo"])))
        );
        assert_eq!(
            indented_code_block("      foo"),
            Ok(("", LeafBlock::IndentedCodeBlock(vec!["  foo"])))
        );
        assert_eq!(
            indented_code_block("    a simple\n      indented code block"),
            Ok((
                "",
                LeafBlock::IndentedCodeBlock(vec!["a simple", "  indented code block"])
            ))
        );
        assert_eq!(
            indented_code_block("    chunk1\n\n    chunk 2\n  \n \n \n    chunk 3"),
            Ok((
                "",
                LeafBlock::IndentedCodeBlock(vec![
                    "chunk1", "", "chunk 2", "  ", " ", " ", "chunk 3"
                ])
            ))
        );
        assert_eq!(
            indented_code_block("    chunk1\n  \n      chunk 2"),
            Ok((
                "",
                LeafBlock::IndentedCodeBlock(vec!["chunk1", "  ", "  chunk 2"])
            ))
        );
        assert_eq!(
            indented_code_block("    foo\nbar"),
            Ok(("bar", LeafBlock::IndentedCodeBlock(vec!["foo"])))
        );
        assert_eq!(
            indented_code_block("      foo\n    bar"),
            Ok(("", LeafBlock::IndentedCodeBlock(vec!["  foo", "bar"])))
        );
        assert_eq!(
            indented_code_block("    foo  "),
            Ok(("", LeafBlock::IndentedCodeBlock(vec!["foo  "])))
        );
    }
}
