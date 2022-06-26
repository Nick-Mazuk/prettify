use super::nodes::Block;
use prettify::{group, hard_line, join, PrettifyDoc};

mod header;
mod leaf;

pub fn create_prettify_doc<'a>(nodes: Vec<Block>) -> PrettifyDoc<'a> {
    group(join(
        nodes
            .into_iter()
            .map(|node| match node {
                Block::Header(size, leaves) => header::format_header(size, leaves),
                Block::Paragraph(leaves) => leaf::format_leaves(leaves),
            })
            .collect(),
        hard_line(),
    ))
}
