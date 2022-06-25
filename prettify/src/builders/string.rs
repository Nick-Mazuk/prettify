use super::super::doc::Doc;
use std::rc::Rc;

pub fn string<'a, S: Into<String>>(str: S) -> Rc<Doc<'a>> {
    Rc::new(Doc::String(str.into()))
}
