use crate::helpers::print_doc_helper;
use crate::{Doc, PrettifyConfig};
use std::boxed::Box;
use std::vec::Vec;

pub fn print_group(doc: &Box<Doc>, output: &mut String, options: &PrettifyConfig) {
    match **separator {
        Doc::String(separator) => {
            for (index, doc) in docs.iter().enumerate() {
                print_doc_helper(doc, output, options);
                if index < docs.len() - 1 {
                    output.push_str(&separator);
                }
            }
        }
        Doc::Children(_) => {
            panic!("print_join expected a separator of type Doc::String, found Doc::Children")
        }
        Doc::Command(_) => {
            panic!("print_join expected a separator of type Doc::String, found Doc::Command")
        }
    }
}
