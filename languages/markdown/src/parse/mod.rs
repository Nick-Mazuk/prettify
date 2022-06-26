use self::block::parse_blocks;
use super::nodes::Block;

mod block;
mod header;
mod leaf;
mod paragraph;

pub fn parse_markdown(markdown: &str) -> nom::IResult<&str, Vec<Block>> {
    // let blocks_results = parse_blocks(markdown);
    // let mut blocks: Vec<Block> = Vec::new();
    // for block_result in blocks_results {
    //     match block_result {
    //         Ok((_, block)) => blocks.push(block),
    //         Err(error) => println!("{}", error),
    //     }
    // }
    // blocks
    parse_blocks(markdown)
}
