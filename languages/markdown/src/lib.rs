extern crate prettify;
extern crate prettify_shared;

mod format;
mod nodes;
mod parse;

use format::create_prettify_doc;
use parse::parse_markdown;

pub fn format_markdown(markdown: &str) -> Result<prettify::PrettifyDoc, &str> {
    match parse_markdown(markdown) {
        Ok((_, nodes)) => Ok(create_prettify_doc(nodes)),
        Err(_) => Err("Invalid markdown"),
    }
}
