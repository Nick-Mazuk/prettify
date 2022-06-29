use nom::{bytes::complete::is_a, combinator::opt, sequence::terminated};

use crate::{nodes::LeafBlock, parse::preliminaries::line_ending};

pub fn blank_line(input: &str) -> nom::IResult<&str, LeafBlock> {
    let result = terminated(opt(is_a(" \t")), line_ending)(input);
    match result {
        Ok((remainder, _)) => Ok((remainder, LeafBlock::BlankLine)),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_line_test() {
        assert_eq!(blank_line(""), Ok(("", LeafBlock::BlankLine)));
        assert_eq!(blank_line("\n"), Ok(("", LeafBlock::BlankLine)));
        assert_eq!(blank_line("    \n"), Ok(("", LeafBlock::BlankLine)));
        assert_eq!(blank_line("   \t\t \n"), Ok(("", LeafBlock::BlankLine)));
    }
}
