use super::super::doc::{Doc, DocCommand};
use std::borrow::Cow;

pub fn mark_as_root(doc: Doc) -> Doc {
    Doc::Command(DocCommand::Root(Box::new(Cow::Owned(doc))))
}
