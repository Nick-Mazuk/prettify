use crate::{
    nodes::LeafBlock,
    parse::preliminaries::{any_until_block_ending, block_ending},
};
use nom::sequence::terminated;

pub fn paragraph(input: &str) -> nom::IResult<&str, LeafBlock> {
    let (remainder, content) = terminated(any_until_block_ending, block_ending)(input)?;
    Ok((remainder, LeafBlock::Paragraph(content.trim())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraph_test() {
        assert_eq!(paragraph("foo"), Ok(("", LeafBlock::Paragraph("foo"))));
        assert_eq!(paragraph("foo\n"), Ok(("", LeafBlock::Paragraph("foo"))));
        assert_eq!(paragraph("foo\n\n"), Ok(("", LeafBlock::Paragraph("foo"))));
        assert_eq!(
            paragraph("foo\n\nbar"),
            Ok(("bar", LeafBlock::Paragraph("foo")))
        );

        // paragraphs can span multiple lines
        assert_eq!(
            paragraph("foo\nbar"),
            Ok(("", LeafBlock::Paragraph("foo\nbar")))
        );
    }
}
