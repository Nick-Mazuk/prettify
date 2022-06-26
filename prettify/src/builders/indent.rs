use super::super::doc::{Doc, DocCommand};
use crate::PrettifyDoc;
use std::rc::Rc;

pub fn indent(doc: PrettifyDoc) -> PrettifyDoc {
    Rc::new(Doc::Command(DocCommand::Indent(doc)))
}
