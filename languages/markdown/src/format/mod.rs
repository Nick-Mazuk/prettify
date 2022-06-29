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
            .filter(|node| match node {
                Block::Leaf(LeafBlock::BlankLine) => false,
                _ => true,
            })
            .map(|node| match node {
                Block::Leaf(LeafBlock::Heading(size, content)) => {
                    header::format_header(size, content)
                }
                Block::Leaf(LeafBlock::ThematicBreak) => concat(vec![string("---"), hard_line()]),
                Block::Leaf(LeafBlock::Paragraph(content)) => paragraph::format_paragraph(content),
                Block::Leaf(LeafBlock::BlankLine) => {
                    panic!("Blank lines are not renderable and should be removed before this point")
                }
            })
            .collect(),
        hard_line(),
    ))
}
