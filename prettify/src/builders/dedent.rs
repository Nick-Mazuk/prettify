use super::super::doc::{AlignAmount, Doc};
use super::align::align;
use std::rc::Rc;

pub fn dedent_to_root(doc: Rc<Doc>) -> Rc<Doc> {
    align(doc, AlignAmount::DedentToRoot)
}

pub fn dedent(doc: Rc<Doc>) -> Rc<Doc> {
    align(doc, AlignAmount::Dedent)
}
