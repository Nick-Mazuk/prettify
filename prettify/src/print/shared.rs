use crate::Doc;
use std::borrow::Cow;

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

pub type Command<'a> = (Indent, Mode, Cow<'a, Doc<'a>>);
pub type LineSuffixes<'a> = Vec<&'a str>;
pub type Out = Vec<String>;
