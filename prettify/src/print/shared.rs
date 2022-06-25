use super::super::doc::Doc;

pub const PRINT_WIDTH: usize = 80;
pub const NEW_LINE: char = '\n';

#[derive(PartialEq, Debug, Clone)]
pub enum IndentKind {
    Indent,
    Dedent,
    StringAlign(String),
    NumberAlign(usize),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Indent {
    pub value: String,
    pub length: usize,
    pub queue: Vec<Indent>,
    pub kind: Option<IndentKind>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Mode {
    Break,
    Flat,
}

#[derive(PartialEq, Debug, Clone)]
pub enum OutKind {
    String(String),
    Cursor,
}

pub type LineSuffixes<'a> = Vec<&'a str>;
pub type Out = Vec<OutKind>;
pub type Command<'a> = (Indent, Mode, Doc<'a>);
pub type Commands<'a> = Vec<Command<'a>>;
pub type GroupModeMap<'a> = std::collections::HashMap<&'a str, Mode>;
