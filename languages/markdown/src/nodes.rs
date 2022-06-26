#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    String(String),
    Heading(usize, Vec<Leaf>),
    Paragraph(Vec<Leaf>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Leaf {
    String(String),
}

pub type Leaves = Vec<Leaf>;
