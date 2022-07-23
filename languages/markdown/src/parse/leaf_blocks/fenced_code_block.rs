use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{eof, recognize},
    multi::{count, many0, many_m_n, many_till},
    sequence::{delimited, preceded, terminated, tuple},
};

use crate::{
    nodes::LeafBlock,
    parse::preliminaries::{any_until_line_ending, line, line_ending, space0, SPACE_STR},
};

pub fn fenced_code_block(input: &str) -> nom::IResult<&str, LeafBlock> {
    let (remainder, (indent, fence)) = tuple((
        many_m_n(0, 3, tag(SPACE_STR)),
        alt((
            recognize(tuple((tag("```"), many0(tag("`"))))),
            recognize(tuple((tag("~~~"), many0(tag("~"))))),
        )),
    ))(input)?;
    let end_fence_char = &fence[0..1];

    let (remainder, info) = alt((
        delimited(space0, tag(""), line_ending),
        delimited(space0, any_until_line_ending, line_ending),
    ))(remainder)?;

    if info.starts_with(end_fence_char) {
        return Err(nom::Err::Error(nom::error::Error {
            code: nom::error::ErrorKind::IsNot,
            input: info,
        }));
    }

    if end_fence_char == "`" && info.contains("`") {
        return Err(nom::Err::Error(nom::error::Error {
            code: nom::error::ErrorKind::IsNot,
            input: info,
        }));
    }

    let (remainder, (lines, _)) = many_till(
        preceded(many_m_n(0, indent.len(), tag(SPACE_STR)), line),
        alt((
            terminated(
                recognize(tuple((
                    many_m_n(0, 3, tag(SPACE_STR)),
                    count(tag(end_fence_char), fence.len()),
                    many0(tag(end_fence_char)),
                ))),
                line_ending,
            ),
            eof,
        )),
    )(remainder)?;

    Ok((remainder, LeafBlock::FencedCodeBlock(info.trim(), lines)))
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify_shared::parse::test_helpers::assert_errors;

    #[test]
    fn fenced_code_block_test() {
        assert_eq!(
            fenced_code_block("```\n<\n >\n```"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["<", " >"])))
        );
        assert_eq!(
            fenced_code_block("~~~\n<\n >\n~~~"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["<", " >"])))
        );
        assert_eq!(
            fenced_code_block("```\naaa\n~~~\n```"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["aaa", "~~~"])))
        );
        assert_eq!(
            fenced_code_block("~~~\naaa\n```\n~~~"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["aaa", "```"])))
        );
        assert_eq!(
            fenced_code_block("````\naaa\n```\n``````"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["aaa", "```"])))
        );
        assert_eq!(
            fenced_code_block("~~~~\naaa\n~~~\n~~~~"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["aaa", "~~~"])))
        );
        assert_eq!(
            fenced_code_block("~~~"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec![])))
        );
        assert_eq!(
            fenced_code_block("`````\n```\naaa"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["```", "aaa"])))
        );
        assert_eq!(
            fenced_code_block("```\n  \n```"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["  "])))
        );
        assert_eq!(
            fenced_code_block("```\n```"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec![])))
        );
        assert_eq!(
            fenced_code_block(" ```\n aaa\naaa\n```"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["aaa", "aaa"])))
        );
        assert_eq!(
            fenced_code_block("  ```\naaa\n  aaa\naaa\n  ```"),
            Ok((
                "",
                LeafBlock::FencedCodeBlock("", vec!["aaa", "aaa", "aaa"])
            ))
        );
        assert_eq!(
            fenced_code_block("   ```\n   aaa\n    aaa\n  aaa\n   ```"),
            Ok((
                "",
                LeafBlock::FencedCodeBlock("", vec!["aaa", " aaa", "aaa"])
            ))
        );
        assert_eq!(
            fenced_code_block("```\naaa\n  ```"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["aaa"])))
        );
        assert_eq!(
            fenced_code_block("   ```\naaa\n  ```"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["aaa"])))
        );
        assert_eq!(
            fenced_code_block("```\naaa\n    ```"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["aaa", "    ```"])))
        );
        assert_eq!(
            fenced_code_block("  ```\naaa\n    ```"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["aaa", "  ```"])))
        );
        assert_eq!(
            fenced_code_block("~~~~~~\naaa\n~~~ ~~"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["aaa", "~~~ ~~"])))
        );
        assert_eq!(
            fenced_code_block("```ruby\ndef foo(x)\n  return 3\nend\n```"),
            Ok((
                "",
                LeafBlock::FencedCodeBlock("ruby", vec!["def foo(x)", "  return 3", "end"])
            ))
        );
        assert_eq!(
            fenced_code_block("~~~ruby~~~\ndef foo(x)\n  return 3\nend\n~~~"),
            Ok((
                "",
                LeafBlock::FencedCodeBlock("ruby~~~", vec!["def foo(x)", "  return 3", "end"])
            ))
        );
        assert_eq!(
            fenced_code_block(
                "~~~~    ruby startline=3 $%@#$\ndef foo(x)\n  return 3\nend\n~~~~~~~"
            ),
            Ok((
                "",
                LeafBlock::FencedCodeBlock(
                    "ruby startline=3 $%@#$",
                    vec!["def foo(x)", "  return 3", "end"]
                )
            ))
        );
        assert_eq!(
            fenced_code_block("````;\n````"),
            Ok(("", LeafBlock::FencedCodeBlock(";", vec![])))
        );
        assert_eq!(
            fenced_code_block("````;\n````"),
            Ok(("", LeafBlock::FencedCodeBlock(";", vec![])))
        );
        assert_eq!(
            fenced_code_block("~~~ aa ``` ~~~\nfoo\n~~~"),
            Ok(("", LeafBlock::FencedCodeBlock("aa ``` ~~~", vec!["foo"])))
        );
        assert_eq!(
            fenced_code_block("```\n``` aaa\n```"),
            Ok(("", LeafBlock::FencedCodeBlock("", vec!["``` aaa"])))
        );
    }

    #[test]
    fn fenced_code_block_fails() {
        assert_errors(fenced_code_block("~~\n<\n >\n~~"));
        assert_errors(fenced_code_block("``\n<\n >\n``"));
        assert_errors(fenced_code_block("``` ```\naaa"));
        assert_errors(fenced_code_block("``` aa ```\nfoo"));
    }
}
