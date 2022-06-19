use super::super::doc::{Doc, DocCommand, DocOptions};
use std::borrow::Cow;

pub fn fill(docs: Vec<Doc>) -> Doc {
    Doc::Command(DocCommand::Fill(
        docs.into_iter().map(Cow::Owned).collect(),
        DocOptions {
            id: "",
            should_break: false,
        },
    ))
}
