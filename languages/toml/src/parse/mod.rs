use crate::nodes::Block;

mod boolean;

pub fn parse_toml(input: &str) -> nom::IResult<&str, Vec<Block>> {
    Ok((input, vec![]))
}
