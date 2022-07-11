use crate::nodes::LeafBlock;

use super::nodes::Block;
use prettify::{concat, group, hard_line, join, string, PrettifyDoc};

mod heading;
mod indented_code_block;
mod paragraph;

pub fn create_prettify_doc(nodes: Vec<Block>) -> PrettifyDoc {
    group(join(
        nodes
            .into_iter()
            .filter(|node| !matches!(node, Block::Leaf(LeafBlock::BlankLine)))
            .map(|node| match node {
                Block::Leaf(LeafBlock::AtxHeading(size, content)) => {
                    heading::format_atx_heading(size, content)
                }
                Block::Leaf(LeafBlock::ThematicBreak) => concat(vec![string("---"), hard_line()]),
                Block::Leaf(LeafBlock::Paragraph(content)) => paragraph::format_paragraph(content),
                Block::Leaf(LeafBlock::SetextHeading(size, content)) => {
                    heading::format_setext_heading(size, content)
                }
                Block::Leaf(LeafBlock::IndentedCodeBlock(code)) => {
                    indented_code_block::format_indented_code_block(code)
                }
                Block::Leaf(LeafBlock::BlankLine) => {
                    panic!("Blank lines are not renderable and should be removed before this point")
                }
            })
            .collect(),
        hard_line(),
    ))
}
