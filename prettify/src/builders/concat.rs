use super::super::doc::Doc;
use std::rc::Rc;

pub fn concat(docs: Vec<Rc<Doc>>) -> Rc<Doc> {
    Rc::new(Doc::Children(docs))
}
