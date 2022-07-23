extern crate nom;
extern crate prettify;

mod format;
mod nodes;
mod parse;

use format::create_prettify_doc;
use parse::parse_toml;

pub fn format_toml(markdown: &str) -> Result<prettify::PrettifyDoc, &str> {
    match parse_toml(markdown) {
        Ok((_, nodes)) => Ok(create_prettify_doc(nodes)),
        Err(_) => Err("Invalid markdown"),
    }
}
