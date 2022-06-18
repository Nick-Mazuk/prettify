use super::super::doc::{Doc, DocCommand};
use std::borrow::Cow;

pub fn indent(doc: Doc) -> Doc {
    Doc::Command(DocCommand::Indent(Box::new(Cow::Owned(doc))))
}
