#[derive(PartialEq, Debug, Clone)]
pub enum LeafBlock<'a> {
    ThematicBreak,
    AtxHeading(usize, &'a str),
    SetextHeading(usize, &'a str),
    Paragraph(&'a str),
    IndentedCodeBlock(Vec<&'a str>),
    BlankLine,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Block<'a> {
    Leaf(LeafBlock<'a>),
}
