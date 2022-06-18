use crate::{Doc, DocCommand};
use std::borrow::Cow;

pub fn indent<'a>(doc: Doc<'a>) -> Doc<'a> {
    Doc::Command(DocCommand::Indent(Box::new(Cow::Owned(doc))))
}
