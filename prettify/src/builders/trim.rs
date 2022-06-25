use super::super::doc::{Doc, DocCommand};
use std::rc::Rc;

pub fn trim<'a>() -> Rc<Doc<'a>> {
    Rc::new(Doc::Command(DocCommand::Trim))
}
