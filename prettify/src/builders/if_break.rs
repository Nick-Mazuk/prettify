use super::super::doc::{Doc, DocCommand};
use std::rc::Rc;

pub fn if_break<'a>(
    break_contents: Rc<Doc<'a>>,
    flat_contents: Rc<Doc<'a>>,
    group_id: String,
) -> Rc<Doc<'a>> {
    Rc::new(Doc::Command(DocCommand::IfBreak(
        break_contents,
        flat_contents,
        group_id,
    )))
}

pub fn indent_if_break(break_contents: Rc<Doc>, group_id: String, negate: bool) -> Rc<Doc> {
    Rc::new(Doc::Command(DocCommand::IndentIfBreak(
        break_contents,
        group_id,
        negate,
    )))
}
