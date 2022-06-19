use super::super::doc::{Doc, DocCommand};

pub fn trim<'a>() -> Doc<'a> {
    Doc::Command(DocCommand::Trim)
}
