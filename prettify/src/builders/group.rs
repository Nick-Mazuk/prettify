use super::super::doc::{Doc, DocCommand, DocOptions};
use std::borrow::Cow;

pub fn group(doc: Doc) -> Doc {
    Doc::Command(DocCommand::Group(
        Box::new(Cow::Owned(doc)),
        Cow::Owned(DocOptions {
            id: "",
            should_break: false,
            expanded_states: vec![],
        }),
    ))
}

pub fn group_with_options<'a>(doc: Doc<'a>, options: DocOptions<'a>) -> Doc<'a> {
    Doc::Command(DocCommand::Group(
        Box::new(Cow::Owned(doc)),
        Cow::Owned(options),
    ))
}

pub fn conditional_group<'a>(docs: Vec<Doc<'a>>, options: DocOptions<'a>) -> Doc<'a> {
    let doc = docs.get(0);
    match doc {
        Some(doc) => Doc::Command(DocCommand::Group(
            Box::new(Cow::Owned(doc.clone())),
            Cow::Owned(DocOptions {
                id: options.id,
                should_break: options.should_break,
                expanded_states: docs,
            }),
        )),
        None => {
            panic!("conditional_group requires at least one doc");
        }
    }
}
