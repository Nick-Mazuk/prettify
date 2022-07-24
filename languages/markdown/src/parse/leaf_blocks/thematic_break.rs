use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::recognize,
    multi::{count, many0, many_m_n},
    sequence::tuple,
};

use crate::{
    nodes::LeafBlock,
    parse::preliminaries::{line_ending, space, space0},
};

pub fn thematic_break(input: &str) -> nom::IResult<&str, LeafBlock> {
    let (remainder, _) = tuple((
        many_m_n(0, 3, space),
        alt((
            tuple((
                count(tuple((tag("-"), space0)), 3),
                many0(alt((recognize(one_of("-")), space))),
            )),
            tuple((
                count(tuple((tag("*"), space0)), 3),
                many0(alt((recognize(one_of("*")), space))),
            )),
            tuple((
                count(tuple((tag("_"), space0)), 3),
                many0(alt((recognize(one_of("_")), space))),
            )),
        )),
        line_ending,
    ))(input)?;
    Ok((remainder, LeafBlock::ThematicBreak))
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettify_shared::assert_errors;

    #[test]
    fn thematic_break_test() {
        let ok_response: nom::IResult<&str, LeafBlock> = Ok(("", LeafBlock::ThematicBreak));
        assert_eq!(thematic_break("---"), ok_response);
        assert_eq!(thematic_break("---\n"), ok_response);
        assert_eq!(thematic_break("---    \n"), ok_response);
        assert_eq!(thematic_break(" ---"), ok_response);
        assert_eq!(thematic_break("  ---"), ok_response);
        assert_eq!(thematic_break("   ---"), ok_response);
        assert_eq!(thematic_break("***"), ok_response);
        assert_eq!(thematic_break("***\n"), ok_response);
        assert_eq!(thematic_break("***    \n"), ok_response);
        assert_eq!(thematic_break(" ***"), ok_response);
        assert_eq!(thematic_break("  ***"), ok_response);
        assert_eq!(thematic_break("   ***"), ok_response);
        assert_eq!(thematic_break("___"), ok_response);
        assert_eq!(thematic_break("___\n"), ok_response);
        assert_eq!(thematic_break("___    \n"), ok_response);
        assert_eq!(thematic_break(" ___"), ok_response);
        assert_eq!(thematic_break("  ___"), ok_response);
        assert_eq!(thematic_break("   ___"), ok_response);
        assert_eq!(thematic_break("- -  -  "), ok_response);
        assert_eq!(thematic_break("* *  *  "), ok_response);
        assert_eq!(thematic_break("_ _  _  "), ok_response);
        assert_eq!(
            thematic_break("_____________________________________"),
            ok_response
        );
        assert_eq!(thematic_break(" - - -"), ok_response);
        assert_eq!(thematic_break(" **  * ** * ** * **"), ok_response);
        assert_eq!(thematic_break("-     -      -      -"), ok_response);
        assert_eq!(thematic_break("- - - -    "), ok_response);
        assert_eq!(thematic_break("* * *"), ok_response);
    }

    #[test]
    fn thematic_break_failure_test() {
        assert_errors(thematic_break("    ---"));
        assert_errors(thematic_break("+++"));
        assert_errors(thematic_break("==="));
        assert_errors(thematic_break("--"));
        assert_errors(thematic_break("**"));
        assert_errors(thematic_break("__"));
        assert_errors(thematic_break("_ _ _ _ a"));
        assert_errors(thematic_break("a------"));
        assert_errors(thematic_break("---a---"));
        assert_errors(thematic_break("*-*"));
        assert_errors(thematic_break("*** -"));
        assert_errors(thematic_break("--- *"));
        assert_errors(thematic_break("___ *"));
    }
}
