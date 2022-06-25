use super::super::doc::{Doc, DocCommand, DocOptions};

pub fn group(doc: Doc) -> Doc {
    Doc::Command(DocCommand::Group(
        Box::new(doc),
        DocOptions {
            id: "",
            should_break: false,
            expanded_states: vec![],
        },
    ))
}

pub fn group_with_options<'a>(doc: Doc<'a>, options: DocOptions<'a>) -> Doc<'a> {
    Doc::Command(DocCommand::Group(Box::new(doc), options))
}

pub fn conditional_group<'a>(docs: Vec<Doc<'a>>, id: &'a str) -> Doc<'a> {
    if docs.is_empty() {
        panic!("conditional_group requires at least one doc");
    }
    let doc = &docs[0];
    Doc::Command(DocCommand::Group(
        Box::new(doc.clone()),
        DocOptions {
            id,
            should_break: false,
            expanded_states: docs,
        },
    ))
}
