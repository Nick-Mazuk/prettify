use super::super::doc::{Doc, DocCommand};

pub fn break_parent<'a>() -> Doc<'a> {
    Doc::Command(DocCommand::BreakParent)
}
