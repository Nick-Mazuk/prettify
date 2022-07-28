use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space0},
    combinator::{eof, opt},
    sequence::tuple,
};
use prettify::{concat, hard_line, string, PrettifyDoc};

pub fn line_end_with_optional_comment(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, (_, comment, _)) = tuple((
        space0,
        opt(tuple((tag("#"), not_line_ending))),
        alt((line_ending, eof)),
    ))(input)?;

    Ok((
        remainder,
        concat(vec![
            match comment {
                Some((_, mut comment)) => {
                    comment = comment.trim();
                    if comment.len() > 0 {
                        string(format!("# {}", comment))
                    } else {
                        string("")
                    }
                }
                None => string(""),
            },
            hard_line(),
        ]),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify_shared::assert_formatted;

    #[test]
    fn line_end_with_optional_comment_test() {
        assert_formatted(
            line_end_with_optional_comment("# comment"),
            ("", "# comment\n"),
        );
        assert_formatted(
            line_end_with_optional_comment("#comment    "),
            ("", "# comment\n"),
        );
        assert_formatted(
            line_end_with_optional_comment("# comment\n"),
            ("", "# comment\n"),
        );
        assert_formatted(
            line_end_with_optional_comment("# comment\n\n"),
            ("\n", "# comment\n"),
        );
        assert_formatted(line_end_with_optional_comment("#\n\n"), ("\n", "\n"));
        assert_formatted(line_end_with_optional_comment("\n\n"), ("\n", "\n"));
        assert_formatted(line_end_with_optional_comment("        \n\n"), ("\n", "\n"));
    }
}
