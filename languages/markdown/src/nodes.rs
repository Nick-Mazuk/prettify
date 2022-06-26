#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Header(usize, Vec<Leaf>),
    Paragraph(Vec<Leaf>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Leaf {
    String(String),
}

pub type Leaves = Vec<Leaf>;
