use super::super::doc::{Doc, DocCommand, LineMode};

pub fn break_parent<'a>() -> Doc<'a> {
    Doc::Command(DocCommand::BreakParent)
}

pub fn hardline_without_break_parent<'a>() -> Doc<'a> {
    Doc::Command(DocCommand::Line(LineMode::Hard))
}
