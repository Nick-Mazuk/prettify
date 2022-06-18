use crate::Doc;
use std::borrow::Cow;

pub fn concat<'a>(docs: &'a Vec<Doc<'a>>) -> Doc<'a> {
    Doc::Children(docs.iter().map(|item| Cow::Borrowed(item)).collect())
}
