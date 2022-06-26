extern crate prettify;

mod format;
mod nodes;
mod parse;

use format::create_prettify_doc;
use parse::parse_markdown;

pub fn format_markdown<'a>(markdown: &str) -> prettify::PrettifyDoc<'a> {
    create_prettify_doc(parse_markdown(markdown))
}
