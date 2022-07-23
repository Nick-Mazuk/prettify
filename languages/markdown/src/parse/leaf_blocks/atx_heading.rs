use nom::{
    bytes::complete::tag,
    combinator::{opt, recognize},
    multi::many_m_n,
    sequence::{delimited, tuple},
};

use crate::{
    nodes::LeafBlock,
    parse::preliminaries::{any_until_line_ending, line_ending, space, space1, SPACE_CHAR},
};

fn trim_content(content: &str) -> &str {
    let content = content.trim();
    for char in content.chars().rev() {
        if char == SPACE_CHAR {
            break;
        }
        if char != '#' {
            return content;
        }
    }
    content.trim_end_matches('#').trim_end()
}

pub fn atx_heading(input: &str) -> nom::IResult<&str, LeafBlock> {
    let (remainder, (hashtags, content)) = delimited(
        many_m_n(0, 3, space),
        tuple((
            many_m_n(1, 6, tag("#")),
            opt(recognize(tuple((space1, any_until_line_ending)))),
        )),
        line_ending,
    )(input)?;
    match content {
        Some(content) => Ok((
            remainder,
            LeafBlock::AtxHeading(hashtags.len(), trim_content(content)),
        )),
        None => Ok((remainder, LeafBlock::AtxHeading(hashtags.len(), ""))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettify_shared::parse::test_helpers::assert_errors;

    #[test]
    fn atx_heading_test() {
        assert_eq!(
            atx_heading("# foo"),
            Ok(("", LeafBlock::AtxHeading(1, "foo")))
        );
        assert_eq!(
            atx_heading("## foo"),
            Ok(("", LeafBlock::AtxHeading(2, "foo")))
        );
        assert_eq!(
            atx_heading("### foo"),
            Ok(("", LeafBlock::AtxHeading(3, "foo")))
        );
        assert_eq!(
            atx_heading("#### foo"),
            Ok(("", LeafBlock::AtxHeading(4, "foo")))
        );
        assert_eq!(
            atx_heading("##### foo"),
            Ok(("", LeafBlock::AtxHeading(5, "foo")))
        );
        assert_eq!(
            atx_heading("###### foo"),
            Ok(("", LeafBlock::AtxHeading(6, "foo")))
        );
        assert_eq!(
            atx_heading("#                  foo                     "),
            Ok(("", LeafBlock::AtxHeading(1, "foo")))
        );
        assert_eq!(
            atx_heading(" # foo"),
            Ok(("", LeafBlock::AtxHeading(1, "foo")))
        );
        assert_eq!(
            atx_heading("  # foo"),
            Ok(("", LeafBlock::AtxHeading(1, "foo")))
        );
        assert_eq!(
            atx_heading("   # foo"),
            Ok(("", LeafBlock::AtxHeading(1, "foo")))
        );
        assert_eq!(
            atx_heading("## foo ##"),
            Ok(("", LeafBlock::AtxHeading(2, "foo")))
        );
        assert_eq!(
            atx_heading("# foo ##################################"),
            Ok(("", LeafBlock::AtxHeading(1, "foo")))
        );
        assert_eq!(
            atx_heading("##### foo ##"),
            Ok(("", LeafBlock::AtxHeading(5, "foo")))
        );
        assert_eq!(
            atx_heading("### foo ###     "),
            Ok(("", LeafBlock::AtxHeading(3, "foo")))
        );
        assert_eq!(
            atx_heading("### foo ### b"),
            Ok(("", LeafBlock::AtxHeading(3, "foo ### b")))
        );
        assert_eq!(
            atx_heading("# foo#"),
            Ok(("", LeafBlock::AtxHeading(1, "foo#")))
        );
        assert_eq!(
            atx_heading("### foo \\###"),
            Ok(("", LeafBlock::AtxHeading(3, "foo \\###")))
        );
        assert_eq!(
            atx_heading("## foo #\\##"),
            Ok(("", LeafBlock::AtxHeading(2, "foo #\\##")))
        );
        assert_eq!(
            atx_heading("# foo \\#"),
            Ok(("", LeafBlock::AtxHeading(1, "foo \\#")))
        );
        assert_eq!(atx_heading("## "), Ok(("", LeafBlock::AtxHeading(2, ""))));
        assert_eq!(atx_heading("#"), Ok(("", LeafBlock::AtxHeading(1, ""))));
        // consumes the newline character
        assert_eq!(
            atx_heading("# Foo\n"),
            Ok(("", LeafBlock::AtxHeading(1, "Foo")))
        );
        assert_eq!(
            atx_heading("# Foo\n\nhello"),
            Ok(("\nhello", LeafBlock::AtxHeading(1, "Foo")))
        );
        assert_eq!(
            atx_heading("# Foo\nhello"),
            Ok(("hello", LeafBlock::AtxHeading(1, "Foo")))
        );
    }

    #[test]
    fn atx_heading_failure_test() {
        assert_errors(atx_heading("####### 7 symbols"));
        assert_errors(atx_heading("#hashtag"));
        assert_errors(atx_heading("\\## foo"));
        assert_errors(atx_heading("    # foo"));
    }
}
