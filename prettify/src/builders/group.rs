use super::super::doc::{Doc, DocCommand, DocOptions};
use std::rc::Rc;

pub fn group(doc: Rc<Doc>) -> Rc<Doc> {
    group_with_options(
        doc,
        DocOptions {
            id: "",
            should_break: false,
            expanded_states: vec![],
        },
    )
}

pub fn group_with_options<'a>(doc: Rc<Doc<'a>>, options: DocOptions<'a>) -> Rc<Doc<'a>> {
    Rc::new(Doc::Command(DocCommand::Group(doc, Rc::new(options))))
}

pub fn conditional_group<'a>(docs: Vec<Rc<Doc<'a>>>, id: &'a str) -> Rc<Doc<'a>> {
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
