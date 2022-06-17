use super::indent::generate_indent;
use super::shared::{Indent, IndentKind};
use crate::PrettifyConfig;

// This version of the method contains many simplifications from the original that
// might prove useful later. The following URL links to the original implementation:
// https://sourcegraph.com/github.com/prettier/prettier/-/blob/src/document/doc-printer.js?L23
pub fn make_align(indent: Indent, width: usize, config: &PrettifyConfig) -> Indent {
    generate_indent(
        indent,
        Indent {
            kind: Some(IndentKind::NumberAlign(width)),
            length: 0,
            queue: Vec::new(),
            value: String::new(),
        },
        config,
    )
}
