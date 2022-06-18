use crate::{AlignAmount, Doc, DocCommand};
use std::borrow::Cow;

pub fn align<'a>(contents: Doc<'a>, amount: AlignAmount) -> Doc<'a> {
    Doc::Command(DocCommand::Align(Box::new(Cow::Owned(contents)), amount))
}
