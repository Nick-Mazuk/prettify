// #[derive(PartialEq, Debug, Clone)]
// pub enum Block {
//     Header(usize, Vec<Leaf>),
//     Paragraph(Vec<Leaf>),
//     EmptyLine,
// }

// #[derive(PartialEq, Debug, Clone)]
// pub enum Leaf {
//     String(String),
// }

// pub type Leaves = Vec<Leaf>;

// new nodes

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LeafBlock<'a> {
    ThematicBreak,
    Heading(usize, &'a str),
    Paragraph(&'a str),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Block<'a> {
    Leaf(LeafBlock<'a>),
}
