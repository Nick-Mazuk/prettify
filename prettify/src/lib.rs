extern crate regex;
pub use doc::*;
// use helpers::print_doc_helper;

// mod command;
mod doc;
mod helpers;
mod print;

pub const PRETTIFY_DEFAULT_CONFIG: PrettifyConfig = PrettifyConfig { tab_width: 4 };

pub fn print<'a>(doc: &'a Doc<'a>) -> String {
    print::print_to_string(doc, &PRETTIFY_DEFAULT_CONFIG)
}

pub fn print_with_config<'a>(doc: &'a Doc<'a>, options: &PrettifyConfig) -> String {
    print::print_to_string(doc, options)
}
