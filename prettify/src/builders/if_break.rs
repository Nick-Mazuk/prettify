use super::super::doc::{Doc, DocCommand};
use crate::PrettifyDoc;
use std::rc::Rc;

pub fn if_break<'a>(
    break_contents: PrettifyDoc<'a>,
    flat_contents: PrettifyDoc<'a>,
    group_id: String,
) -> PrettifyDoc<'a> {
    Rc::new(Doc::Command(DocCommand::IfBreak(
        break_contents,
        flat_contents,
        group_id,
    )))
}

pub fn indent_if_break(break_contents: PrettifyDoc, group_id: String, negate: bool) -> PrettifyDoc {
    Rc::new(Doc::Command(DocCommand::IndentIfBreak(
        break_contents,
        group_id,
        negate,
    )))
}
