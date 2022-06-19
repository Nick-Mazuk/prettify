use super::super::doc::{Doc, DocCommand};

pub fn line_suffix(str: &str) -> Doc {
    Doc::Command(DocCommand::LineSuffix(str))
}

pub fn line_suffix_boundary<'a>() -> Doc<'a> {
    Doc::Command(DocCommand::LineSuffixBoundary)
}
