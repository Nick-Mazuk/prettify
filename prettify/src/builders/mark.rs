use super::super::doc::{Doc, DocCommand};
use std::rc::Rc;

pub fn mark_as_root(doc: Rc<Doc>) -> Rc<Doc> {
    Rc::new(Doc::Command(DocCommand::Root(doc)))
}
