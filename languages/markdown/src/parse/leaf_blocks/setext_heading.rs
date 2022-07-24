use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{not, peek, recognize},
    multi::{many1, many_m_n, many_till},
    sequence::{delimited, tuple},
};

use crate::{
    nodes::LeafBlock,
    parse::preliminaries::{any_until_line_ending, line_ending, space, space0},
};

fn heading_underline(input: &str) -> nom::IResult<&str, usize> {
    let (remainder, underline) = delimited(
        many_m_n(0, 3, space),
        recognize(alt((many1(tag("=")), many1(tag("-"))))),
        tuple((space0, line_ending)),
    )(input)?;
    Ok((remainder, if underline.contains('=') { 1 } else { 2 }))
}

pub fn setext_heading(input: &str) -> nom::IResult<&str, LeafBlock> {
    let result = tuple((
        recognize(many_till(
            delimited(
                many_m_n(0, 3, space),
                recognize(tuple((not(space), any_until_line_ending))),
                line_ending,
            ),
            peek(heading_underline),
        )),
        heading_underline,
    ))(input);
    match result {
        Ok((remainder, (content, size))) => {
            Ok((remainder, LeafBlock::SetextHeading(size, content.trim())))
        }
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettify_shared::assert_errors;

    #[test]
    fn setext_heading_test() {
        assert_eq!(
            setext_heading("Foo *bar*\n========="),
            Ok(("", LeafBlock::SetextHeading(1, "Foo *bar*")))
        );
        assert_eq!(
            setext_heading("Foo *bar*\n---------"),
            Ok(("", LeafBlock::SetextHeading(2, "Foo *bar*")))
        );
        assert_eq!(
            setext_heading("Foo *bar*\n---------\n"),
            Ok(("", LeafBlock::SetextHeading(2, "Foo *bar*")))
        );
        assert_eq!(
            setext_heading("Foo *bar*\n---------\n\n"),
            Ok(("\n", LeafBlock::SetextHeading(2, "Foo *bar*")))
        );
        assert_eq!(
            setext_heading("Foo *bar*\n---------\nhello"),
            Ok(("hello", LeafBlock::SetextHeading(2, "Foo *bar*")))
        );
        assert_eq!(
            setext_heading("Foo *bar*\n---------\n\nhello"),
            Ok(("\nhello", LeafBlock::SetextHeading(2, "Foo *bar*")))
        );
        assert_eq!(
            setext_heading("Foo *bar\nbaz*\n===="),
            Ok(("", LeafBlock::SetextHeading(1, "Foo *bar\nbaz*")))
        );
        assert_eq!(
            setext_heading("  Foo *bar\nbaz*\t\n===="),
            Ok(("", LeafBlock::SetextHeading(1, "Foo *bar\nbaz*")))
        );
        assert_eq!(
            setext_heading("Foo\n-------------------------"),
            Ok(("", LeafBlock::SetextHeading(2, "Foo")))
        );
        assert_eq!(
            setext_heading("Foo\n="),
            Ok(("", LeafBlock::SetextHeading(1, "Foo")))
        );
        assert_eq!(
            setext_heading("   Foo\n---"),
            Ok(("", LeafBlock::SetextHeading(2, "Foo")))
        );
        assert_eq!(
            setext_heading("  Foo\n---"),
            Ok(("", LeafBlock::SetextHeading(2, "Foo")))
        );
        assert_eq!(
            setext_heading("  Foo\n  ==="),
            Ok(("", LeafBlock::SetextHeading(1, "Foo")))
        );
        assert_eq!(
            setext_heading("  Foo\n   ===            "),
            Ok(("", LeafBlock::SetextHeading(1, "Foo")))
        );
        assert_eq!(
            setext_heading("  Foo\n   ----      "),
            Ok(("", LeafBlock::SetextHeading(2, "Foo")))
        );
        assert_eq!(
            setext_heading("Foo  \n-----"),
            Ok(("", LeafBlock::SetextHeading(2, "Foo")))
        );
        assert_eq!(
            setext_heading("Foo\\\n----"),
            Ok(("", LeafBlock::SetextHeading(2, "Foo\\")))
        );
        assert_eq!(
            setext_heading("`Foo\n----\n`"),
            Ok(("`", LeafBlock::SetextHeading(2, "`Foo")))
        );
        assert_eq!(
            setext_heading("<a title=\"a lot\n---\nof dashes\"/>"),
            Ok((
                "of dashes\"/>",
                LeafBlock::SetextHeading(2, "<a title=\"a lot")
            ))
        );
        assert_eq!(
            setext_heading("Foo\nBar\n===="),
            Ok(("", LeafBlock::SetextHeading(1, "Foo\nBar")))
        );
    }

    #[test]
    fn setext_heading_failure_test() {
        assert_errors(setext_heading("    Foo\n    ---"));
        assert_errors(setext_heading("    Foo\n---"));
        assert_errors(setext_heading("Foo\n    ---"));
        assert_errors(setext_heading("Foo\n= ="));
        assert_errors(setext_heading("Foo\n--- -"));
    }
}
