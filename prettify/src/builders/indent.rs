use super::super::doc::{Doc, DocCommand};

pub fn indent(doc: Doc) -> Doc {
    Doc::Command(DocCommand::Indent(Box::new(doc)))
}
