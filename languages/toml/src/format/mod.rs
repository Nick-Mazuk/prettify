use crate::nodes::{Node, Table};
use prettify::{group, hard_line, join, line_suffix, string, PrettifyDoc};

pub fn create_prettify_doc(blocks: Vec<Table>) -> PrettifyDoc {
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
                            Node::Comment(comment) => line_suffix(comment),
                        })
                        .collect(),
                    hard_line(),
                )
            })
            .collect(),
        hard_line(),
    ))
}
