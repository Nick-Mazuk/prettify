use super::super::doc::{AlignAmount, Doc, DocCommand};
use std::rc::Rc;

pub fn align(contents: Rc<Doc>, amount: AlignAmount) -> Rc<Doc> {
    Rc::new(Doc::Command(DocCommand::Align(contents, amount)))
}

pub fn add_alignment_to_doc(doc: Rc<Doc>, size: usize, tab_width: usize) -> Rc<Doc> {
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
