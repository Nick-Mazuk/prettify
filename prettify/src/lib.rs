extern crate regex;
pub use builders::*;
use doc::{Doc, PrettifyConfig};

mod builders;
mod doc;
mod print;

pub const PRETTIFY_DEFAULT_CONFIG: PrettifyConfig = PrettifyConfig { tab_width: 4 };

pub fn print(doc: Doc) -> String {
    print::print_to_string(doc, &PRETTIFY_DEFAULT_CONFIG)
}

pub fn print_with_config(doc: Doc, options: &PrettifyConfig) -> String {
    print::print_to_string(doc, options)
}
