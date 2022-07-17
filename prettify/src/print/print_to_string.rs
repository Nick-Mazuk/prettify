use super::super::doc::{Doc, DocCommand, LineMode, PrettifyConfig};
use super::align::make_align;
use super::fits::fits;
use super::indent::make_indent;
use super::shared::{
    Command, Commands, GroupModeMap, Indent, LineSuffixes, Mode, Out, OutKind, NEW_LINE,
    PRINT_WIDTH,
};
use super::trim::trim;
use crate::indent as build_indent;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

fn root_indent() -> Rc<Indent> {
    Rc::new(Indent {
        value: String::new(),
        length: 0,
        queue: Vec::new(),
        kind: None,
    })
}

// This function is long for three reasons:
// 1. Using a stack is substantially faster than making it recursive
// 2. Breaking each match block into separate functions requires the use of Cow<'a, Doc<'a>
//    which creates a lot of extra .clone() calls.
// 3. Using Cows leads to a lot of extra boilerplate and unintuitive patterns.
pub fn print_to_string<'a>(doc: Rc<Doc<'a>>, config: &PrettifyConfig) -> String {
    let mut pos: usize = 0;
    let mut should_remeasure = false;
    let mut out: Out = vec![];
    let mut line_suffixes: LineSuffixes<'a> = vec![];
    let mut group_mode_map: GroupModeMap = HashMap::new();
    let mut commands: Commands = vec![(root_indent(), Mode::Break, doc)];

    while !commands.is_empty() {
        let (indent, mode, doc) = commands.pop().unwrap();

        match doc.borrow() {
            Doc::String(string) => {
                out.push(OutKind::String(string.to_string()));
                pos += string.len();
            }
            Doc::Children(children) => {
                for child in children.iter().rev() {
                    commands.push((Rc::clone(&indent), mode, Rc::clone(child)));
                }
            }
            Doc::Command(command) => match command {
                DocCommand::Indent(contents) => {
                    commands.push((make_indent(indent, config), mode, Rc::clone(contents)));
                }
                DocCommand::Align(contents, width) => {
                    commands.push((
                        make_align(indent, width.clone(), config),
                        mode,
                        Rc::clone(contents),
                    ));
                }
                DocCommand::Trim => {
                    pos -= trim(&mut out);
                }
                DocCommand::Group(contents, options) => {
                    let mut should_insert_into_map = true;
                    let next_mode = Mode::Flat;
                    if mode == Mode::Flat && !should_remeasure {
                        commands.push((
                            Rc::clone(&indent),
                            if options.should_break {
                                Mode::Break
                            } else {
                                Mode::Flat
                            },
                            Rc::clone(contents),
                        ));
                    } else {
                        should_remeasure = false;
                        let next: Command = (Rc::clone(&indent), Mode::Flat, Rc::clone(contents));
                        let remainder = PRINT_WIDTH - pos;
                        let has_line_suffix = !line_suffixes.is_empty();
                        if !options.should_break
                            && fits(
                                &next,
                                &commands,
                                remainder,
                                options,
                                has_line_suffix,
                                false,
                                config,
                            )
                        {
                            commands.push(next);
                        } else if !options.expanded_states.is_empty() {
                            let most_expanded = options.expanded_states.last().unwrap();
                            if options.should_break {
                                commands.push((
                                    Rc::clone(&indent),
                                    Mode::Break,
                                    Rc::clone(most_expanded),
                                ));
                            } else {
                                let expanded_states = &options.expanded_states;
                                for i in 1..(expanded_states.len() + 1) {
                                    if i >= expanded_states.len() {
                                        commands.push((
                                            Rc::clone(&indent),
                                            Mode::Flat,
                                            Rc::clone(most_expanded),
                                        ));
                                        should_insert_into_map = false;
                                        break;
                                    } else {
                                        let command = (
                                            Rc::clone(&indent),
                                            next_mode,
                                            Rc::clone(&expanded_states[i]),
                                        );
                                        if fits(
                                            &command,
                                            &commands,
                                            remainder,
                                            options,
                                            has_line_suffix,
                                            false,
                                            config,
                                        ) {
                                            commands.push(command);
                                            should_insert_into_map = false;
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            commands.push((indent, Mode::Break, Rc::clone(contents)));
                        }
                    }
                    if should_insert_into_map {
                        group_mode_map.insert(options.id, next_mode);
                    }
                }
                DocCommand::Fill(contents, doc_options) => {
                    let remainder = PRINT_WIDTH - pos;
                    if !contents.is_empty() {
                        let content = &contents[0];
                        let contents_command_flat: Command =
                            (Rc::clone(&indent), Mode::Flat, Rc::clone(content));
                        let contents_command_break: Command =
                            (Rc::clone(&indent), Mode::Break, Rc::clone(content));
                        let content_fits = fits(
                            &contents_command_flat,
                            &Vec::new(),
                            remainder,
                            doc_options,
                            !line_suffixes.is_empty(),
                            true,
                            config,
                        );
                        if contents.len() == 1 {
                            if content_fits {
                                commands.push(contents_command_flat);
                            } else {
                                commands.push(contents_command_break);
                            }
                        } else {
                            let whitespace = &contents[1];
                            let whitespace_command_flat: Command =
                                (Rc::clone(&indent), Mode::Flat, Rc::clone(whitespace));
                            let whitespace_command_break: Command =
                                (Rc::clone(&indent), Mode::Break, Rc::clone(whitespace));

                            if contents.len() == 2 {
                                if content_fits {
                                    commands.push(contents_command_flat);
                                    commands.push(whitespace_command_flat);
                                } else {
                                    commands.push(contents_command_break);
                                    commands.push(whitespace_command_break);
                                }
                            } else {
                                let mut cloned_contents = contents.clone();
                                let item_0 = cloned_contents.remove(0);
                                let item_1 = cloned_contents.remove(0);
                                let first_and_second_content_flat_command: Command = (
                                    Rc::clone(&indent),
                                    Mode::Flat,
                                    Rc::new(Doc::Children(vec![item_0, item_1])),
                                );
                                let first_and_second_content_fits = fits(
                                    &first_and_second_content_flat_command,
                                    &Vec::new(),
                                    remainder,
                                    doc_options,
                                    !line_suffixes.is_empty(),
                                    true,
                                    config,
                                );
                                let remaining_command: Command = (
                                    indent,
                                    mode,
                                    Rc::new(Doc::Command(DocCommand::Fill(
                                        cloned_contents,
                                        Rc::clone(doc_options),
                                    ))),
                                );

                                commands.push(remaining_command);

                                if first_and_second_content_fits {
                                    commands.push(whitespace_command_flat);
                                    commands.push(contents_command_flat);
                                } else if content_fits {
                                    commands.push(whitespace_command_break);
                                    commands.push(contents_command_flat);
                                } else {
                                    commands.push(whitespace_command_break);
                                    commands.push(contents_command_break);
                                }
                            }
                        }
                    }
                }
                DocCommand::IfBreak(break_contents, flat_contents, group_id) => {
                    let group_mode = match group_mode_map.get(group_id as &str) {
                        Some(mapped_mode) => mapped_mode,
                        None => &mode,
                    };
                    match group_mode {
                        Mode::Break => {
                            commands.push((indent, mode, Rc::clone(break_contents)));
                        }
                        Mode::Flat => {
                            commands.push((indent, mode, Rc::clone(flat_contents)));
                        }
                    }
                }
                DocCommand::IndentIfBreak(contents, group_id, negate) => {
                    let group_mode = match group_mode_map.get(group_id as &str) {
                        Some(mapped_mode) => mapped_mode,
                        None => &mode,
                    };
                    match group_mode {
                        Mode::Break => {
                            if *negate {
                                commands.push((indent, mode, Rc::clone(contents)));
                            } else {
                                commands.push((indent, mode, build_indent(Rc::clone(contents))));
                            }
                        }
                        Mode::Flat => {
                            if *negate {
                                commands.push((indent, mode, build_indent(Rc::clone(contents))));
                            } else {
                                commands.push((indent, mode, Rc::clone(contents)));
                            }
                        }
                    }
                }
                DocCommand::LineSuffix(contents) => {
                    line_suffixes.push(contents);
                }
                DocCommand::LineSuffixBoundary => commands.push((
                    indent,
                    mode,
                    Rc::new(Doc::Command(DocCommand::Line(LineMode::Hard))),
                )),
                DocCommand::Line(line_mode) => {
                    if mode == Mode::Flat && *line_mode == LineMode::Auto {
                        out.push(OutKind::String(String::from(" ")));
                        pos += 1;
                    } else if mode != Mode::Flat || *line_mode != LineMode::Soft {
                        if *line_mode == LineMode::Hard || *line_mode == LineMode::HardLiteral {
                            should_remeasure = true;
                        }
                        if line_suffixes.is_empty() {
                            if *line_mode == LineMode::HardLiteral {
                                out.push(OutKind::String(NEW_LINE.to_string()));
                                pos = 0;
                            } else {
                                trim(&mut out);
                                out.push(OutKind::String(NEW_LINE.to_string() + &indent.value));
                                pos = indent.length;
                            }
                        } else {
                            commands.push((Rc::clone(&indent), mode, doc));
                            for suffix in line_suffixes.iter().rev() {
                                commands.push((
                                    Rc::clone(&indent),
                                    mode,
                                    Rc::new(Doc::String(suffix.to_string())),
                                ));
                            }
                            line_suffixes.clear();
                        }
                    }
                }
                DocCommand::Cursor => {
                    out.push(OutKind::Cursor);
                }
                DocCommand::BreakParent => {
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
        if let OutKind::String(string) = kind {
            result.push_str(&string)
        }
    }
    result
}
