use super::super::doc::{Doc, DocCommand, DocOptions};
use crate::PrettifyDoc;
use std::rc::Rc;

pub fn group(doc: PrettifyDoc) -> PrettifyDoc {
    group_with_options(
        doc,
        DocOptions {
            id: "",
            should_break: false,
            expanded_states: vec![],
        },
    )
}

pub fn group_with_options<'a>(doc: PrettifyDoc<'a>, options: DocOptions<'a>) -> PrettifyDoc<'a> {
    Rc::new(Doc::Command(DocCommand::Group(doc, Rc::new(options))))
}

pub fn conditional_group<'a>(docs: Vec<PrettifyDoc<'a>>, id: &'a str) -> PrettifyDoc<'a> {
    if docs.is_empty() {
        panic!("conditional_group requires at least one doc");
    }
    let doc = &docs[0];
    group_with_options(
        Rc::clone(doc),
        DocOptions {
            id,
            should_break: false,
            expanded_states: docs,
        },
    )
}
