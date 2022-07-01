#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LeafBlock<'a> {
    ThematicBreak,
    AtxHeading(usize, &'a str),
    Paragraph(&'a str),
    BlankLine,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Block<'a> {
    Leaf(LeafBlock<'a>),
}
