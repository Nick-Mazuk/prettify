use super::super::doc::{Doc, DocCommand};
use std::rc::Rc;

pub fn break_parent<'a>() -> Rc<Doc<'a>> {
    Rc::new(Doc::Command(DocCommand::BreakParent))
}
