use super::nodes::{Leaf, Node};

pub fn parse_markdown(markdown: &str) -> Vec<Node> {
    vec![Node::Header(
        1,
        vec![Leaf::String("Hello world".to_string())],
    )]
}
