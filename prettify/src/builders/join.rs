use crate::Doc;
use std::borrow::Cow;

pub fn join<'a>(docs: Vec<Doc<'a>>, separator: Doc<'a>) -> Doc<'a> {
    let mut children: Vec<Cow<'a, Doc<'a>>> = Vec::new();
    for (index, doc) in docs.into_iter().enumerate() {
        if index != 0 {
            children.push(Cow::Owned(separator.clone()));
        }
        children.push(Cow::Owned(doc))
    }
    Doc::Children(children)
}
