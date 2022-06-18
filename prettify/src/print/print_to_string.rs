use super::align::make_align;
use super::fill::process_fill;
use super::group::process_group;
use super::indent::make_indent;
use super::line::process_line;
use super::shared::{Command, Indent, LineSuffixes, Mode, Out};
use super::trim::trim;
use crate::{AlignAmount, Doc, DocCommand, LineMode, PrettifyConfig};
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

type Commands<'a> = Vec<Command<'a>>;
type GroupModeMap<'a> = std::collections::HashMap<&'a str, Mode>;

pub fn print_to_string<'a>(doc: Doc<'a>, config: &PrettifyConfig) -> String {
    let mut pos: usize = 0;
    let mut should_remeasure = false;
    let mut out: Out = vec![];
    let mut line_suffixes: LineSuffixes<'a> = vec![];
    let mut group_mode_map: GroupModeMap = HashMap::new();
    let mut commands: Commands = vec![(root_indent(), Mode::Break, Cow::Owned(doc))];

    while commands.len() > 0 {
        let (indent, mode, doc) = commands.pop().unwrap();

        let owned_doc = match doc {
            Cow::Owned(owned_doc) => owned_doc,
            Cow::Borrowed(borrowed_doc) => borrowed_doc.clone(),
        };

        match owned_doc {
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
                    commands.push((make_indent(indent, config), mode, *contents));
                }
                DocCommand::Align(contents, width) => {
                    commands.push((make_align(indent, width, config), mode, *contents));
                }
                DocCommand::Trim => {
                    pos -= trim(&mut out);
                }
                DocCommand::Group(contents, doc_options) => {
                    process_group(
                        *contents,
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
                        &doc_options,
                        &mode,
                    );
                }
                DocCommand::LineSuffix(contents) => {
                    line_suffixes.push(contents);
                }
                DocCommand::LineSuffixBoundary => commands.push((
                    indent,
                    mode,
                    Cow::Owned(Doc::Command(DocCommand::Line(LineMode::Hard))),
                )),
                DocCommand::Line(line_mode) => {
                    process_line(line_mode, &mode, &mut out, &mut pos, &mut should_remeasure)
                }
            },
        }
    }

    out.join("")
}
