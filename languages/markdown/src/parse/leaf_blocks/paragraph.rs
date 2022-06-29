use crate::{
    nodes::LeafBlock,
    parse::preliminaries::{any_until_line_ending, line_ending},
};
use nom::sequence::terminated;

pub fn paragraph(input: &str) -> nom::IResult<&str, LeafBlock> {
    let result = terminated(any_until_line_ending, line_ending)(input);
    match result {
        Ok((remainder, content)) => Ok((remainder, LeafBlock::Paragraph(content.trim()))),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraph_test() {
        assert_eq!(paragraph("foo"), Ok(("", LeafBlock::Paragraph("foo"))));
    }
}
