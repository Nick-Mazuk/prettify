use super::super::doc::{Doc, DocCommand, DocOptions};
use std::rc::Rc;

pub fn fill(docs: Vec<Rc<Doc>>) -> Rc<Doc> {
    Rc::new(Doc::Command(DocCommand::Fill(
        docs,
        DocOptions {
            id: "",
            should_break: false,
            expanded_states: vec![],
        },
    )))
}
