use crate::nodes::Node;
use nom::branch::alt;
use nom::bytes::complete::tag;

pub fn boolean(input: &str) -> nom::IResult<&str, Node> {
    let (remainder, value) = alt((tag("true"), tag("false")))(input)?;
    match value {
        "true" => Ok((remainder, Node::Boolean(true))),
        "false" => Ok((remainder, Node::Boolean(false))),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_boolean() {
        assert_eq!(boolean("true"), Ok(("", Node::Boolean(true))));
        assert_eq!(boolean("false"), Ok(("", Node::Boolean(false))));
        assert_eq!(boolean("truefalse"), Ok(("false", Node::Boolean(true))));
    }
}
