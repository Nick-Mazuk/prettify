use super::super::doc::{Doc, DocCommand};

pub fn cursor<'a>() -> Doc<'a> {
    Doc::Command(DocCommand::Cursor)
}
