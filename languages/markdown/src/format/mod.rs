use super::nodes::Node;
use prettify::{group, hard_line, join, PrettifyDoc};

mod header;
mod leaf;

pub fn create_prettify_doc<'a>(nodes: Vec<Node>) -> PrettifyDoc<'a> {
    group(join(
        nodes
            .into_iter()
            .map(|node| match node {
                Node::Header(size, leaves) => header::format_header(size, leaves),
                Node::Paragraph(leaves) => leaf::format_leaves(leaves),
            })
            .collect(),
        hard_line(),
    ))
}
