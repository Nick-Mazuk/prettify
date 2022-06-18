use crate::Doc;

pub fn string<'a>(str: &'a str) -> Doc<'a> {
    Doc::String(str)
}
