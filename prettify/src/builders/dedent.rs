use super::super::doc::AlignAmount;
use super::align::align;
use crate::PrettifyDoc;

pub fn dedent_to_root(doc: PrettifyDoc) -> PrettifyDoc {
    align(doc, AlignAmount::DedentToRoot)
}

pub fn dedent(doc: PrettifyDoc) -> PrettifyDoc {
    align(doc, AlignAmount::Dedent)
}
