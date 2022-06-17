use super::align::make_align;
use super::fill::process_fill;
use super::group::process_group;
use super::indent::make_indent;
use super::shared::{Command, Indent, Mode};
use super::trim::trim;
use crate::{Doc, DocCommand, PrettifyConfig};
use std::collections::HashMap;

fn root_indent() -> Indent {
    Indent {
        value: String::new(),
        length: 0,
        queue: Vec::new(),
        kind: None,
    }
}

pub fn print_to_string<'a>(doc: &mut Doc<'a>, config: &PrettifyConfig) -> String {
    let mut pos: usize = 0;
    let mut should_remeasure = false;
    let mut out: Vec<String> = vec![];
    let mut line_suffixes: Vec<String> = vec![];
    let mut group_mode_map = HashMap::new();
    let mut commands: Vec<Command<'a>> = vec![(root_indent(), Mode::Break, doc)];

    while commands.len() > 0 {
        let (indent, mode, doc) = commands.pop().unwrap();

        match doc {
            Doc::String(string) => {
                out.push(string.to_string());
                pos += string.len();
            }
            Doc::Children(children) => {
                for child in children {
                    commands.push((indent.clone(), mode, child));
                }
            }
            Doc::Command(command) => match command {
                DocCommand::Indent(contents) => {
                    commands.push((make_indent(indent, config), mode, contents));
                }
                DocCommand::Align(width, contents) => {
                    commands.push((make_align(indent, *width, config), mode, contents));
                }
                DocCommand::Trim => {
                    pos -= trim(&mut out);
                }
                DocCommand::Group(contents, doc_options) => {
                    process_group(
                        contents,
                        doc_options,
                        &mut commands,
                        &mut line_suffixes,
                        indent,
                        &mode,
                        &mut group_mode_map,
                        &mut pos,
                        &mut should_remeasure,
                    );
                }
                DocCommand::Fill(contents, doc_options) => {
                    process_fill(
                        &mut commands,
                        contents,
                        &indent,
                        &pos,
                        &line_suffixes,
                        doc_options,
                        &mode,
                        &mut doc,
                    );
                }
            },
        }
    }

    out.join("")
}
