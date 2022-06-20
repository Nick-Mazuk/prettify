use super::super::doc::{AlignAmount, Doc, DocCommand};
use std::borrow::Cow;

pub fn align(contents: Doc, amount: AlignAmount) -> Doc {
    Doc::Command(DocCommand::Align(Box::new(Cow::Owned(contents)), amount))
}

pub fn add_alignment_to_doc(doc: Doc, size: usize, tab_width: usize) -> Doc {
    let mut aligned = doc;
    if size > 0 {
        // Use indent to add tabs for all the levels of tabs we need
        for i in 0..(size / tab_width) {
            aligned = align(aligned, AlignAmount::Spaces(i * tab_width));
        }
        // Use align for all the spaces that are needed
        aligned = align(aligned, AlignAmount::Spaces(size % tab_width));
        // size is absolute from 0 and not relative to the current indentation
        aligned = align(aligned, AlignAmount::DedentToRoot);
    }
    aligned
}
