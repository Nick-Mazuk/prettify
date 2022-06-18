use super::super::doc::{Doc, DocCommand, DocOptions};
use std::borrow::Cow;

pub fn group(doc: Doc) -> Doc {
    Doc::Command(DocCommand::Group(
        Box::new(Cow::Owned(doc)),
        Cow::Owned(DocOptions {
            id: "",
            should_break: false,
        }),
    ))
}

pub fn group_with_options<'a>(doc: Doc<'a>, options: DocOptions<'a>) -> Doc<'a> {
    Doc::Command(DocCommand::Group(
        Box::new(Cow::Owned(doc)),
        Cow::Owned(options),
    ))
}
