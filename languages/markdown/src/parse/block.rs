use super::header::header;
use crate::nodes::Block;
use nom::branch::alt;
use regex::Regex;

pub type ParsedBlocks<'a> = Vec<nom::IResult<&'a str, Block>>;

pub fn parse_blocks(input: &str) -> ParsedBlocks {
    let block_separator = Regex::new(r"(\n[\n]+)").expect("Invalid regex");
    block_separator
        .split(input)
        .into_iter()
        .map(|item| {
            let item = item.trim();
            block(item)
        })
        .collect()
}

pub fn block(input: &str) -> nom::IResult<&str, Block> {
    alt((header, header))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::nodes::Leaf;

    #[test]
    fn block_header() {
        assert_eq!(
            block("# hello world"),
            Ok((
                "",
                Block::Header(1, vec![Leaf::String("hello world".to_string())])
            ))
        );
    }
}
