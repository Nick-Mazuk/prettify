use super::super::doc::Doc;

pub fn join<'a>(docs: Vec<Doc<'a>>, separator: Doc<'a>) -> Doc<'a> {
    Doc::Children(join_to_vector(docs, separator))
}

pub fn join_to_vector<'a>(docs: Vec<Doc<'a>>, separator: Doc<'a>) -> Vec<Doc<'a>> {
    let mut children: Vec<Doc<'a>> = Vec::new();
    for (index, doc) in docs.into_iter().enumerate() {
        if index != 0 {
            children.push(separator.clone());
        }
        children.push(doc)
    }
    children
}
