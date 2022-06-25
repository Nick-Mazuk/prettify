use super::super::doc::{Doc, DocCommand};
use std::rc::Rc;

pub fn line_suffix(str: &str) -> Rc<Doc> {
    Rc::new(Doc::Command(DocCommand::LineSuffix(str)))
}

pub fn line_suffix_boundary<'a>() -> Rc<Doc<'a>> {
    Rc::new(Doc::Command(DocCommand::LineSuffixBoundary))
}
