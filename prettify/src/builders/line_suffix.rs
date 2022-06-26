use super::super::doc::{Doc, DocCommand};
use crate::PrettifyDoc;
use std::rc::Rc;

pub fn line_suffix(str: &str) -> PrettifyDoc {
    Rc::new(Doc::Command(DocCommand::LineSuffix(str)))
}

pub fn line_suffix_boundary<'a>() -> PrettifyDoc<'a> {
    Rc::new(Doc::Command(DocCommand::LineSuffixBoundary))
}
