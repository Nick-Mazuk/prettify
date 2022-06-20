use super::super::doc::{Doc, DocCommand, LineMode, PrettifyConfig};
use super::align::make_align;
use super::fill::process_fill;
use super::group::process_group;
use super::if_break::{process_if_break, process_indent_if_break};
use super::indent::make_indent;
use super::line::process_line;
use super::shared::{Commands, GroupModeMap, Indent, LineSuffixes, Mode, Out, OutKind};
use super::trim::trim;
use std::borrow::Borrow;
use std::borrow::Cow;

use std::collections::HashMap;

fn root_indent() -> Indent {
    Indent {
        value: String::new(),
        length: 0,
        queue: Vec::new(),
        kind: None,
    }
}

pub fn print_to_string<'a>(doc: Doc<'a>, config: &PrettifyConfig) -> String {
    let mut pos: usize = 0;
    let mut should_remeasure = false;
    let mut out: Out = vec![];
    let mut line_suffixes: LineSuffixes<'a> = vec![];
    let mut group_mode_map: GroupModeMap = HashMap::new();
    let mut commands: Commands = vec![(root_indent(), Mode::Break, Cow::Owned(doc))];

    while !commands.is_empty() {
        let (indent, mode, doc) = commands.pop().unwrap();

        let borrowed_doc: &Doc = doc.borrow();

        match borrowed_doc.clone() {
            Doc::String(string) => {
                out.push(OutKind::String(string.to_string()));
                pos += string.len();
            }
            Doc::Children(children) => {
                for child in children.into_iter().rev() {
                    commands.push((indent.clone(), mode, child));
                }
            }
            Doc::Command(command) => match command {
                DocCommand::Indent(contents) => {
                    commands.push((make_indent(indent, config), mode, *contents));
                }
                DocCommand::Align(contents, width) => {
                    commands.push((make_align(indent, width, config), mode, *contents));
                }
                DocCommand::Trim => {
                    pos -= trim(&mut out);
                }
                DocCommand::Group(contents, options) => {
                    process_group(
                        *contents,
                        options,
                        &mut commands,
                        &mut line_suffixes,
                        indent,
                        &mode,
                        &mut group_mode_map,
                        &mut pos,
                        &mut should_remeasure,
                        config,
                    );
                }
                DocCommand::Fill(contents, doc_options) => {
                    process_fill(
                        &mut commands,
                        contents,
                        &indent,
                        &pos,
                        &line_suffixes,
                        &doc_options,
                        &mode,
                        config,
                    );
                }
                DocCommand::IfBreak(break_contents, flat_contents, group_id) => {
                    process_if_break(
                        break_contents,
                        flat_contents,
                        group_id,
                        &mode,
                        &group_mode_map,
                        indent,
                        &mut commands,
                    );
                }
                DocCommand::IndentIfBreak(contents, group_id, negate) => process_indent_if_break(
                    contents,
                    group_id,
                    negate,
                    &mode,
                    &group_mode_map,
                    indent,
                    &mut commands,
                ),
                DocCommand::LineSuffix(contents) => {
                    line_suffixes.push(contents);
                }
                DocCommand::LineSuffixBoundary => commands.push((
                    indent,
                    mode,
                    Cow::Owned(Doc::Command(DocCommand::Line(LineMode::Hard))),
                )),
                DocCommand::Line(line_mode) => process_line(
                    line_mode,
                    &mode,
                    &mut out,
                    &mut pos,
                    &mut should_remeasure,
                    &mut line_suffixes,
                    &mut commands,
                    indent,
                    doc,
                ),
                DocCommand::Cursor => {
                    out.push(OutKind::Cursor);
                }
                DocCommand::BreakParent | DocCommand::Root(_) => {
                    // ignore
                }
            },
        }
    }

    transform_out_to_string(out)
}

fn transform_out_to_string(out: Out) -> String {
    let mut result = String::new();
    for kind in out.into_iter() {
        match kind {
            OutKind::String(string) => result.push_str(&string),
            _ => {}
        }
    }
    result
}
