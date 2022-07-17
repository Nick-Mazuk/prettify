use super::super::doc::{Doc, DocCommand};
use crate::PrettifyDoc;
use std::rc::Rc;

pub fn if_break<'a, S: Into<String>>(
    break_contents: PrettifyDoc<'a>,
    flat_contents: PrettifyDoc<'a>,
    group_id: S,
) -> PrettifyDoc<'a> {
    Rc::new(Doc::Command(DocCommand::IfBreak(
        break_contents,
        flat_contents,
        group_id.into(),
    )))
}

pub fn indent_if_break<S: Into<String>>(
    break_contents: PrettifyDoc,
    group_id: S,
    negate: bool,
) -> PrettifyDoc {
    Rc::new(Doc::Command(DocCommand::IndentIfBreak(
        break_contents,
        group_id.into(),
        negate,
    )))
}
