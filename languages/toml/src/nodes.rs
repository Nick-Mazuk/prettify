#[derive(PartialEq, Debug, Clone)]
pub struct Key<'a> {
    pub value: &'a str,
    pub quoted: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub enum IntegerKind {
    Decimal,
    Hexadecimal,
    Octal,
    Binary,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Integer<'a> {
    pub kind: IntegerKind,
    pub value: &'a str,
    pub is_negative: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Node<'a> {
    Boolean(bool),
    Comment(&'a str),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Table<'a> {
    pub nodes: Vec<Node<'a>>,
    pub key: Vec<Key<'a>>,
    pub repeated: bool,
}
