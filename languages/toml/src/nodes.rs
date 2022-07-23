#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Boolean(bool),
}

pub struct Block<'a> {
    pub nodes: Vec<Node>,
    pub name: Option<&'a str>,
}
