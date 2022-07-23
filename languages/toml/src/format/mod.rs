use crate::nodes::{Block, Node};
use prettify::{group, hard_line, join, string, PrettifyDoc};

pub fn create_prettify_doc(blocks: Vec<Block>) -> PrettifyDoc {
    group(join(
        blocks
            .iter()
            .map(|block| {
                join(
                    block
                        .nodes
                        .iter()
                        .map(|node| match *node {
                            Node::Boolean(value) => string(value.to_string()),
                        })
                        .collect(),
                    hard_line(),
                )
            })
            .collect(),
        hard_line(),
    ))
}
