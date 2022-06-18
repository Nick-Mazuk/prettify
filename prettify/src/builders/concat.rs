use super::super::doc::Doc;
use std::borrow::Cow;

pub fn concat(docs: Vec<Doc>) -> Doc {
    Doc::Children(docs.into_iter().map(Cow::Owned).collect())
}
