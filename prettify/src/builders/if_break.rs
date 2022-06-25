use super::super::doc::{Doc, DocCommand};

pub fn if_break<'a>(break_contents: Doc<'a>, flat_contents: Doc<'a>, group_id: String) -> Doc<'a> {
    Doc::Command(DocCommand::IfBreak(
        Box::new(break_contents),
        Box::new(flat_contents),
        group_id,
    ))
}

pub fn indent_if_break(break_contents: Doc, group_id: String, negate: bool) -> Doc {
    Doc::Command(DocCommand::IndentIfBreak(
        Box::new(break_contents),
        group_id,
        negate,
    ))
}
