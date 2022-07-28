use nom::bytes::complete::tag;
use nom::{branch::alt, combinator::map};
use prettify::{string, PrettifyDoc};

pub fn boolean(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    map(alt((tag("true"), tag("false"))), string)(input)
}

#[cfg(test)]
mod test {
    use prettify_shared::assert_formatted;

    use super::*;

    #[test]
    fn test_boolean() {
        assert_formatted(boolean("true"), ("", "true"));
        assert_formatted(boolean("false"), ("", "false"));
        assert_formatted(boolean("truefalse"), ("false", "true"));
    }
}
