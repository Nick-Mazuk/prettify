use crate::nodes::Table;

mod boolean;
mod key;

pub fn parse_toml(input: &str) -> nom::IResult<&str, Vec<Table>> {
    Ok((input, vec![]))
}
