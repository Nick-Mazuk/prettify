use crate::Doc;
use std::borrow::Cow;

pub fn concat<'a>(docs: Vec<Doc<'a>>) -> Doc<'a> {
    Doc::Children(docs.into_iter().map(|item| Cow::Owned(item)).collect())
}
