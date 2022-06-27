extern crate prettify;

mod format;
mod nodes;
mod parse;

use format::create_prettify_doc;
use parse::parse_markdown;

pub fn format_markdown<'a>(markdown: &'a str) -> Result<prettify::PrettifyDoc<'a>, &'a str> {
    match parse_markdown(markdown) {
        Ok((_, nodes)) => Ok(create_prettify_doc(nodes)),
        Err(_) => Err("Invalid markdown"),
    }
}
