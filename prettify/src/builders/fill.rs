use super::super::doc::{Doc, DocCommand, DocOptions};

pub fn fill(docs: Vec<Doc>) -> Doc {
    Doc::Command(DocCommand::Fill(
        docs,
        DocOptions {
            id: "",
            should_break: false,
            expanded_states: vec![],
        },
    ))
}
