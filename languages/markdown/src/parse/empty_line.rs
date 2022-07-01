use crate::nodes::Block;
use nom::bytes::complete::tag;

pub fn empty_line(input: &str) -> nom::IResult<&str, Block> {
    let result = tag("")(input);
    match result {
        Ok((remainder, _)) => Ok((remainder, Block::EmptyLine)),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line_test() {
        assert_eq!(empty_line(""), Ok(("", Block::EmptyLine)));
        assert_eq!(empty_line("\n"), Ok(("\n", Block::EmptyLine)));
    }
}
