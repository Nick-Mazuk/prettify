use super::super::doc::Doc;
use std::rc::Rc;

pub fn join<'a>(docs: Vec<Rc<Doc<'a>>>, separator: Rc<Doc<'a>>) -> Rc<Doc<'a>> {
    Rc::new(Doc::Children(join_to_vector(docs, separator)))
}

pub fn join_to_vector<'a>(docs: Vec<Rc<Doc<'a>>>, separator: Rc<Doc<'a>>) -> Vec<Rc<Doc<'a>>> {
    let mut children: Vec<Rc<Doc<'a>>> = Vec::new();
    for (index, doc) in docs.into_iter().enumerate() {
        if index != 0 {
            children.push(Rc::clone(&separator));
        }
        children.push(doc)
    }
    children
}
