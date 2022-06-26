use crate::nodes::{Leaf, Leaves};
use nom::{
    branch::alt,
    bytes::complete::take_until,
    combinator::{eof, peek, rest},
    sequence::terminated,
};

use super::block::block_end;

// use super::block::block_end;

pub fn leaves(input: &str) -> nom::IResult<&str, Leaves> {
    let result = string(input);
    match result {
        Ok((remainder, leaves)) => Ok((remainder, vec![leaves])),
        Err(error) => Err(error),
    }
}

fn string(input: &str) -> nom::IResult<&str, Leaf> {
    let result = alt((take_until("\n"), rest))(input);
    match result {
        Ok((remainder, consumed)) => Ok((remainder, Leaf::String(consumed.to_string()))),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaves_string() {
        assert_eq!(
            leaves("hello world"),
            Ok(("", vec![Leaf::String("hello world".to_string())]))
        );
    }
}
