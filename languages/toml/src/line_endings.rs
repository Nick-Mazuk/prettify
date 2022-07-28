use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space0},
    combinator::{eof, map, opt},
    sequence::{preceded, tuple},
};
use prettify::{concat, hard_line, string, PrettifyDoc};

fn comment(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    map(preceded(tag("#"), not_line_ending), |result: &str| {
        let trimmed = result.trim();
        if trimmed.len() > 0 {
            string(format!("# {}", trimmed))
        } else {
            string("")
        }
    })(input)
}

pub fn line_end_with_optional_comment(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, (_, comment, _)) =
        tuple((space0, opt(comment), alt((line_ending, eof))))(input)?;

    Ok((
        remainder,
        concat(vec![
            string(" "),
            match comment {
                Some(comment) => comment,
                None => string(""),
            },
            hard_line(),
        ]),
    ))
}

pub fn blank_line_with_optional_comment(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, (_, comment, _)) =
        tuple((space0, opt(comment), alt((line_ending, eof))))(input)?;

    Ok((
        remainder,
        concat(vec![
            match comment {
                Some(comment) => comment,
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
            ("", " # comment\n"),
        );
        assert_formatted(
            line_end_with_optional_comment("#comment    "),
            ("", " # comment\n"),
        );
        assert_formatted(
            line_end_with_optional_comment("# comment\n"),
            ("", " # comment\n"),
        );
        assert_formatted(
            line_end_with_optional_comment("# comment\n\n"),
            ("\n", " # comment\n"),
        );
        assert_formatted(line_end_with_optional_comment("#\n\n"), ("\n", "\n"));
        assert_formatted(line_end_with_optional_comment("\n\n"), ("\n", "\n"));
        assert_formatted(line_end_with_optional_comment("        \n\n"), ("\n", "\n"));
    }

    #[test]
    fn blank_line_with_optional_comment_test() {
        assert_formatted(
            blank_line_with_optional_comment("# comment"),
            ("", "# comment\n"),
        );
        assert_formatted(
            blank_line_with_optional_comment("#comment    "),
            ("", "# comment\n"),
        );
        assert_formatted(
            blank_line_with_optional_comment("   # comment  \n"),
            ("", "# comment\n"),
        );
        assert_formatted(
            blank_line_with_optional_comment("# comment\n\n"),
            ("\n", "# comment\n"),
        );
        assert_formatted(blank_line_with_optional_comment("#\n\n"), ("\n", "\n"));
        assert_formatted(blank_line_with_optional_comment("\n\n"), ("\n", "\n"));
        assert_formatted(
            blank_line_with_optional_comment("        \n\n"),
            ("\n", "\n"),
        );
    }
}
