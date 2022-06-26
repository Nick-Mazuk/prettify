extern crate prettify;

mod format;
mod nodes;
mod parse;

use format::create_prettify_doc;
use parse::parse_markdown;
use prettify::string;

pub fn format_markdown<'a>(markdown: &str) -> prettify::PrettifyDoc<'a> {
    match parse_markdown(markdown) {
        Ok((_, nodes)) => create_prettify_doc(nodes),
        Err(_) => string(markdown),
    }
}
