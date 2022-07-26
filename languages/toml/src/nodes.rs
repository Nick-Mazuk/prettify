#[derive(PartialEq, Debug, Clone)]
pub enum Node<'a> {
    Boolean(bool),
    Comment(&'a str),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Table<'a> {
    pub nodes: Vec<Node<'a>>,
    pub repeated: bool,
}
