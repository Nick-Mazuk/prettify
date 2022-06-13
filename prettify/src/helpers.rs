// use crate::command::print_command;
// use crate::{Doc, PrettifyConfig};

// #[derive(PartialEq, Debug, Clone, Copy)]
// pub struct Context {
//     indent_level: usize,
// }

// pub fn print_doc_helper(doc: &Doc, output: &mut String, options: &PrettifyConfig) {
//     match doc {
//         Doc::Command(command) => print_command(command, output, options),
//         Doc::String(string) => output.push_str(&string),
//         Doc::Children(children) => {
//             for child in children {
//                 print_doc_helper(child, output, options);
//             }
//         }
//     };
// }
