use super::super::doc::{AlignAmount, Doc};
use super::align::align;

pub fn dedent_to_root(doc: Doc) -> Doc {
    align(doc, AlignAmount::DedentToRoot)
}

pub fn dedent(doc: Doc) -> Doc {
    align(doc, AlignAmount::Dedent)
}
