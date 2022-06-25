use super::super::doc::{Doc, DocCommand};

pub fn mark_as_root(doc: Doc) -> Doc {
    Doc::Command(DocCommand::Root(Box::new(doc)))
}
