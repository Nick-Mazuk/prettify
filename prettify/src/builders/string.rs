use super::super::doc::Doc;

pub fn string<'a, S: Into<String>>(str: S) -> Doc<'a> {
    Doc::String(str.into())
}
