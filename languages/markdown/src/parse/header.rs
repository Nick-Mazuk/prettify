use super::leaf::leaves;
use crate::nodes::Block;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::sequence::tuple;

pub fn header(input: &str) -> nom::IResult<&str, Block> {
    let result = tuple((many1(tag("#")), space1, leaves))(input);
    match result {
        Ok((remainder, (tags, _, leaves))) => Ok((remainder, Block::Header(tags.len(), leaves))),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::Leaf;

    #[test]
    fn header_test() {
        assert_eq!(
            header("# hello world"),
            Ok((
                "",
                Block::Header(1, vec![Leaf::String("hello world".to_string())])
            ))
        );
    }
}
