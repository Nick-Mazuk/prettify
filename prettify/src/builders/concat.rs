use super::super::doc::Doc;
use crate::PrettifyDoc;
use std::rc::Rc;

pub fn concat(docs: Vec<PrettifyDoc>) -> PrettifyDoc {
    Rc::new(Doc::Children(docs))
}
