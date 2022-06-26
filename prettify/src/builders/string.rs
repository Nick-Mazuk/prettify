use super::super::doc::Doc;
use crate::PrettifyDoc;
use std::rc::Rc;

pub fn string<'a, S: Into<String>>(str: S) -> PrettifyDoc<'a> {
    Rc::new(Doc::String(str.into()))
}
