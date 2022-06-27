use crate::nodes::LeafBlock;

use super::nodes::Block;
use prettify::{concat, group, hard_line, join, string, PrettifyDoc};

mod header;
mod paragraph;
// mod leaf;

pub fn create_prettify_doc<'a>(nodes: Vec<Block<'a>>) -> PrettifyDoc<'a> {
    group(join(
        nodes
            .into_iter()
            .map(|node| match node {
                Block::Leaf(LeafBlock::Heading(size, content)) => {
                    header::format_header(size, content)
                }
                Block::Leaf(LeafBlock::ThematicBreak) => concat(vec![string("---"), hard_line()]),
                Block::Leaf(LeafBlock::Paragraph(content)) => paragraph::format_paragraph(content),
            })
            .collect(),
        hard_line(),
    ))
}
