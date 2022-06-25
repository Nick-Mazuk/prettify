use super::super::doc::{Doc, DocCommand};
use std::rc::Rc;

pub fn indent(doc: Rc<Doc>) -> Rc<Doc> {
    Rc::new(Doc::Command(DocCommand::Indent(doc)))
}
