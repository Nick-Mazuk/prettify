use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{opt, recognize},
    multi::{many_m_n, many_till},
    sequence::{delimited, tuple},
};

use crate::{
    nodes::LeafBlock,
    parse::preliminaries::{line_ending, space, space1, SPACE_CHAR},
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
    let result = delimited(
        many_m_n(0, 3, space),
        tuple((
            many_m_n(1, 6, tag("#")),
            opt(recognize(tuple((space1, many_till(anychar, line_ending))))),
        )),
        line_ending,
    )(input);
    match result {
        Ok((remainder, (hashtags, Some(content)))) => Ok((
            remainder,
            LeafBlock::Heading(hashtags.len(), trim_content(content)),
        )),
        Ok((remainder, (hashtags, None))) => {
            Ok((remainder, LeafBlock::Heading(hashtags.len(), "")))
        }
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::helpers::assert_errors;

    #[test]
    fn atx_heading_test() {
        assert_eq!(atx_heading("# foo"), Ok(("", LeafBlock::Heading(1, "foo"))));
        assert_eq!(
            atx_heading("## foo"),
            Ok(("", LeafBlock::Heading(2, "foo")))
        );
        assert_eq!(
            atx_heading("### foo"),
            Ok(("", LeafBlock::Heading(3, "foo")))
        );
        assert_eq!(
            atx_heading("#### foo"),
            Ok(("", LeafBlock::Heading(4, "foo")))
        );
        assert_eq!(
            atx_heading("##### foo"),
            Ok(("", LeafBlock::Heading(5, "foo")))
        );
        assert_eq!(
            atx_heading("###### foo"),
            Ok(("", LeafBlock::Heading(6, "foo")))
        );
        assert_eq!(
            atx_heading("#                  foo                     "),
            Ok(("", LeafBlock::Heading(1, "foo")))
        );
        assert_eq!(
            atx_heading(" # foo"),
            Ok(("", LeafBlock::Heading(1, "foo")))
        );
        assert_eq!(
            atx_heading("  # foo"),
            Ok(("", LeafBlock::Heading(1, "foo")))
        );
        assert_eq!(
            atx_heading("   # foo"),
            Ok(("", LeafBlock::Heading(1, "foo")))
        );
        assert_eq!(
            atx_heading("## foo ##"),
            Ok(("", LeafBlock::Heading(2, "foo")))
        );
        assert_eq!(
            atx_heading("# foo ##################################"),
            Ok(("", LeafBlock::Heading(1, "foo")))
        );
        assert_eq!(
            atx_heading("##### foo ##"),
            Ok(("", LeafBlock::Heading(5, "foo")))
        );
        assert_eq!(
            atx_heading("### foo ###     "),
            Ok(("", LeafBlock::Heading(3, "foo")))
        );
        assert_eq!(
            atx_heading("### foo ### b"),
            Ok(("", LeafBlock::Heading(3, "foo ### b")))
        );
        assert_eq!(
            atx_heading("# foo#"),
            Ok(("", LeafBlock::Heading(1, "foo#")))
        );
        assert_eq!(
            atx_heading("### foo \\###"),
            Ok(("", LeafBlock::Heading(3, "foo \\###")))
        );
        assert_eq!(
            atx_heading("## foo #\\##"),
            Ok(("", LeafBlock::Heading(2, "foo #\\##")))
        );
        assert_eq!(
            atx_heading("# foo \\#"),
            Ok(("", LeafBlock::Heading(1, "foo \\#")))
        );
        assert_eq!(atx_heading("## "), Ok(("", LeafBlock::Heading(2, ""))));
        assert_eq!(atx_heading("#"), Ok(("", LeafBlock::Heading(1, ""))));
    }

    #[test]
    fn atx_heading_failure_test() {
        assert_errors(atx_heading("####### 7 symbols"));
        assert_errors(atx_heading("#hashtag"));
        assert_errors(atx_heading("\\## foo"));
        assert_errors(atx_heading("    # foo"));
    }
}
