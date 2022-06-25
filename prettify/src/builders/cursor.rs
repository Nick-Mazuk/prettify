use super::super::doc::{Doc, DocCommand};
use std::rc::Rc;

pub fn cursor<'a>() -> Rc<Doc<'a>> {
    Rc::new(Doc::Command(DocCommand::Cursor))
}
