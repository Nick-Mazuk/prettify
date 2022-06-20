use super::super::doc::{Doc, DocCommand};
use std::borrow::Cow;

pub fn if_break<'a>(break_contents: Doc<'a>, flat_contents: Doc<'a>, group_id: String) -> Doc<'a> {
    Doc::Command(DocCommand::IfBreak(
        Box::new(Cow::Owned(break_contents)),
        Box::new(Cow::Owned(flat_contents)),
        group_id,
    ))
}

pub fn indent_if_break<'a>(break_contents: Doc<'a>, group_id: String, negate: bool) -> Doc<'a> {
    Doc::Command(DocCommand::IndentIfBreak(
        Box::new(Cow::Owned(break_contents)),
        group_id,
        negate,
    ))
}
