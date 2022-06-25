use super::super::doc::Doc;

pub fn concat(docs: Vec<Doc>) -> Doc {
    Doc::Children(docs)
}
