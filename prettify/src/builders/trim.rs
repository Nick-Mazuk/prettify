use super::super::doc::{Doc, DocCommand};
use crate::PrettifyDoc;
use std::rc::Rc;

pub fn trim<'a>() -> PrettifyDoc<'a> {
    Rc::new(Doc::Command(DocCommand::Trim))
}
