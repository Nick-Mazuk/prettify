extern crate regex;
pub use doc::*;
// use helpers::print_doc_helper;

// mod command;
mod doc;
mod helpers;
mod print;

pub const PRETTIFY_DEFAULT_CONFIG: PrettifyConfig = PrettifyConfig { tab_width: 4 };

pub fn print(doc: &mut Doc) -> String {
    print::print_to_string(doc, &PRETTIFY_DEFAULT_CONFIG)
}

pub fn print_with_config(doc: &mut Doc, options: &PrettifyConfig) -> String {
    print::print_to_string(doc, options)
}
