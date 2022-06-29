use self::leaf_blocks::{atx_heading, paragraph, thematic_break};
use super::nodes::Block;
use nom::{branch::alt, combinator::eof, multi::many_till};

// mod block;
// mod empty_line;
// mod header;
mod helpers;
// mod leaf;
mod leaf_blocks;
// mod paragraph;
mod preliminaries;

fn leaf_block_as_block(input: &str) -> nom::IResult<&str, Block> {
    let result = alt((atx_heading, thematic_break, paragraph))(input);
    match result {
        Ok((remainder, block)) => Ok((remainder, Block::Leaf(block))),
        Err(error) => Err(error),
    }
}

pub fn parse_markdown<'a>(markdown: &'a str) -> nom::IResult<&'a str, Vec<Block<'a>>> {
    let result = many_till(leaf_block_as_block, eof)(markdown);
    match result {
        Ok((remainder, (blocks, _))) => Ok((remainder, blocks)),
        Err(error) => Err(error),
    }
}
