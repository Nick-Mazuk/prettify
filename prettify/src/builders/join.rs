use super::super::doc::Doc;
use crate::PrettifyDoc;
use std::rc::Rc;

pub fn join<'a>(docs: Vec<PrettifyDoc<'a>>, separator: PrettifyDoc<'a>) -> PrettifyDoc<'a> {
    Rc::new(Doc::Children(join_to_vector(docs, separator)))
}

pub fn join_to_vector<'a>(
    docs: Vec<PrettifyDoc<'a>>,
    separator: PrettifyDoc<'a>,
) -> Vec<PrettifyDoc<'a>> {
    let mut children: Vec<PrettifyDoc<'a>> = Vec::new();
    for (index, doc) in docs.into_iter().enumerate() {
        if index != 0 {
            children.push(Rc::clone(&separator));
        }
        children.push(doc)
    }
    children
}
