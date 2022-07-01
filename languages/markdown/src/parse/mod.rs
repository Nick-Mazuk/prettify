use self::leaf_blocks::{atx_heading, blank_line, paragraph, thematic_break};
use super::nodes::Block;
use nom::{branch::alt, combinator::eof, multi::many_till};

mod helpers;
mod leaf_blocks;
mod preliminaries;

fn leaf_block_as_block(input: &str) -> nom::IResult<&str, Block> {
    let result = alt((blank_line, atx_heading, thematic_break, paragraph))(input);
    match result {
        Ok((remainder, block)) => Ok((remainder, Block::Leaf(block))),
        Err(error) => Err(error),
    }
}

pub fn parse_markdown(markdown: &str) -> nom::IResult<&str, Vec<Block>> {
    let result = many_till(leaf_block_as_block, eof)(markdown);
    match result {
        Ok((remainder, (blocks, _))) => Ok((remainder, blocks)),
        Err(error) => Err(error),
    }
}
