use super::super::doc::Doc;

pub fn join<'a>(docs: Vec<Doc<'a>>, separator: Doc<'a>) -> Doc<'a> {
    let mut children: Vec<Doc<'a>> = Vec::new();
    for (index, doc) in docs.into_iter().enumerate() {
        if index != 0 {
            children.push(separator.clone());
        }
        children.push(doc)
    }
    Doc::Children(children)
}
