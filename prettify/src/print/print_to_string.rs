use super::align::make_align;
use super::fits::fits;
use super::group::process_group;
use super::indent::make_indent;
use super::shared::{Command, Indent, Mode, PRINT_WIDTH};
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

pub fn print_to_string(doc: &Doc, config: &PrettifyConfig) -> String {
    let mut pos: usize = 0;
    let mut should_remeasure = false;
    let mut out: Vec<String> = vec![];
    let mut line_suffixes: Vec<String> = vec![];
    // let mut group_mode_map = Rc::new(RefCell::new(HashMap::new()));
    let mut group_mode_map = HashMap::new();
    let mut commands: Vec<Command> = vec![(root_indent(), Mode::Break, doc)];

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
                // Fills each line with as much code as possible before moving to a new
                // line with the same indentation.
                //
                // Expects doc.parts to be an array of alternating content and
                // whitespace. The whitespace contains the linebreaks.
                //
                // For example:
                //   ["I", line, "love", line, "monkeys"]
                // or
                //   [{ type: group, ... }, softline, { type: group, ... }]
                //
                // It uses this parts structure to handle three main layout cases:
                // * The first two content items fit on the same line without
                //   breaking
                //   -> output the first content item and the whitespace "flat".
                // * Only the first content item fits on the line without breaking
                //   -> output the first content item "flat" and the whitespace with
                //   "break".
                // * Neither content item fits on the line without breaking
                //   -> output the first content item and the whitespace with "break".
                DocCommand::Fill(contents, doc_options) => {
                    let remainder = PRINT_WIDTH - pos;
                    if contents.len() > 0 {
                        let content = &contents[0];
                        let whitespace = &contents[1];
                        let command_flat = (indent.clone(), Mode::Flat, content);
                        let command_break = (indent.clone(), Mode::Break, content);
                        let content_fits = fits(
                            &command_flat,
                            &Vec::new(),
                            remainder,
                            doc_options,
                            line_suffixes.len() > 0,
                            true,
                        );
                    }
                }
            },
        }
    }

    out.join("")
}
