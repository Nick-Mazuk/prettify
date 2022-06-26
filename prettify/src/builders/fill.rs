use super::super::doc::{Doc, DocCommand, DocOptions};
use crate::PrettifyDoc;
use std::rc::Rc;

pub fn fill(docs: Vec<PrettifyDoc>) -> PrettifyDoc {
    Rc::new(Doc::Command(DocCommand::Fill(
        docs,
        Rc::new(DocOptions {
            id: "",
            should_break: false,
            expanded_states: vec![],
        }),
    )))
}
