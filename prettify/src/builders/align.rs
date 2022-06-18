use super::super::doc::{AlignAmount, Doc, DocCommand};
use std::borrow::Cow;

pub fn align(contents: Doc, amount: AlignAmount) -> Doc {
    Doc::Command(DocCommand::Align(Box::new(Cow::Owned(contents)), amount))
}
