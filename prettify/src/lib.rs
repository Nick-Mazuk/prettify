pub use doc::*;
use helpers::print_doc_helper;

mod command;
mod doc;
mod helpers;

pub const PRETTIFY_DEFAULT_CONFIG: PrettifyConfig = PrettifyConfig { tab_width: 4 };

pub fn print(doc: &Doc) -> String {
    print_with_config(doc, &PRETTIFY_DEFAULT_CONFIG)
}

pub fn print_with_config(doc: &Doc, options: &PrettifyConfig) -> String {
    let mut output = String::new();
    print_doc_helper(doc, &mut output, options);
    output
}
