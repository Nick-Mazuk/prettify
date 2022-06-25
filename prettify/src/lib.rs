extern crate regex;
pub use builders::*;
pub use doc::AlignAmount;
use doc::{Doc, PrettifyConfig};
use std::rc::Rc;

mod builders;
mod doc;
mod print;

pub const PRETTIFY_DEFAULT_CONFIG: PrettifyConfig = PrettifyConfig { tab_width: 4 };

pub fn print(doc: Rc<Doc>) -> String {
    print::print_to_string(doc, &PRETTIFY_DEFAULT_CONFIG)
}

pub fn print_with_config(doc: Rc<Doc>, options: &PrettifyConfig) -> String {
    print::print_to_string(doc, options)
}
