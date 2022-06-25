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
    let mut commands: Commands = vec![(root_indent(), Mode::Break, doc)];
    let mut loop_count = 0;

    while !commands.is_empty() {
        println!("loop_count: {}", loop_count);
        loop_count += 1;
        println!("commands: {:#?}\nout: {:#?}\n\n", commands, out);
        let (indent, mode, doc) = commands.pop().unwrap();

        match doc.clone() {
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
                    if mode == Mode::Flat && !should_remeasure {
                        commands.push((
                            indent.clone(),
                            if options.should_break {
                                Mode::Break
                            } else {
                                Mode::Flat
                            },
                            *contents.clone(),
                        ));
                    };
                    should_remeasure = false;
                    let mut next_mode = Mode::Flat;
                    let next: Command = (indent.clone(), next_mode, *contents.clone());
                    let remainder = PRINT_WIDTH - pos;
                    let has_line_suffix = !line_suffixes.is_empty();
                    let mut should_insert_into_map = true;
                    if options.should_break || options.expanded_states.is_empty() {
                        if !options.should_break
                            && fits(
                                &next,
                                &commands,
                                remainder,
                                &options,
                                has_line_suffix,
                                false,
                                config,
                            )
                        {
                            commands.push(next);
                        } else {
                            should_remeasure = true;
                            next_mode = Mode::Break;
                            commands.push((indent, Mode::Break, *contents));
                        }
                    } else {
                        let expanded_states = &options.expanded_states;
                        for i in 0..options.expanded_states.len() {
                            let option = expanded_states[i].clone();
                            let command = (indent.clone(), next_mode, option);
                            if fits(
                                &command,
                                &commands,
                                remainder,
                                &options,
                                has_line_suffix,
                                false,
                                config,
                            ) {
                                commands.push(command);
                                should_insert_into_map = false;
                            }
                        }
                    }
                    if should_insert_into_map {
                        group_mode_map.insert(options.id, next_mode);
                    }
                }
                DocCommand::Fill(mut contents, doc_options) => {
                    let remainder = PRINT_WIDTH - pos;
                    if !contents.is_empty() {
                        let content = &contents[0];
                        let contents_command_flat: Command =
                            (indent.clone(), Mode::Flat, content.clone());
                        let contents_command_break: Command =
                            (indent.clone(), Mode::Break, content.clone());
                        let content_fits = fits(
                            &contents_command_flat,
                            &Vec::new(),
                            remainder,
                            &doc_options,
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
                                (indent.clone(), Mode::Flat, whitespace.clone());
                            let whitespace_command_break: Command =
                                (indent.clone(), Mode::Break, whitespace.clone());

                            if contents.len() == 2 {
                                if content_fits {
                                    commands.push(contents_command_flat);
                                    commands.push(whitespace_command_flat);
                                } else {
                                    commands.push(contents_command_break);
                                    commands.push(whitespace_command_break);
                                }
                            } else {
                                let item_0 = contents.remove(0);
                                let item_1 = contents.remove(0);
                                let first_and_second_content_flat_command: Command = (
                                    indent.clone(),
                                    Mode::Flat,
                                    Doc::Children(vec![item_0, item_1]),
                                );
                                let first_and_second_content_fits = fits(
                                    &first_and_second_content_flat_command,
                                    &Vec::new(),
                                    remainder,
                                    &doc_options,
                                    !line_suffixes.is_empty(),
                                    true,
                                    config,
                                );
                                let remaining_command: Command = (
                                    indent,
                                    mode,
                                    Doc::Command(DocCommand::Fill(contents, doc_options)),
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
                    let group_mode = match group_mode_map.get(&group_id as &str) {
                        Some(mapped_mode) => mapped_mode,
                        None => &mode,
                    };
                    match group_mode {
                        Mode::Break => {
                            commands.push((indent, mode, *break_contents));
                        }
                        Mode::Flat => {
                            commands.push((indent, mode, *flat_contents));
                        }
                    }
                }
                DocCommand::IndentIfBreak(contents, group_id, negate) => {
                    let group_mode = match group_mode_map.get(&group_id as &str) {
                        Some(mapped_mode) => mapped_mode,
                        None => &mode,
                    };
                    match group_mode {
                        Mode::Break => {
                            if negate {
                                commands.push((indent, mode, *contents));
                            } else {
                                commands.push((indent, mode, build_indent(*contents)));
                            }
                        }
                        Mode::Flat => {
                            if negate {
                                commands.push((indent, mode, build_indent(*contents)));
                            } else {
                                commands.push((indent, mode, *contents));
                            }
                        }
                    }
                }
                DocCommand::LineSuffix(contents) => {
                    line_suffixes.push(contents);
                }
                DocCommand::LineSuffixBoundary => {
                    commands.push((indent, mode, Doc::Command(DocCommand::Line(LineMode::Hard))))
                }
                DocCommand::Line(line_mode) => {
                    if mode == Mode::Flat && line_mode == LineMode::Auto {
                        out.push(OutKind::String(String::from(" ")));
                        pos += 1;
                    } else if mode != Mode::Flat || line_mode != LineMode::Soft {
                        if line_mode == LineMode::Hard || line_mode == LineMode::HardLiteral {
                            should_remeasure = true;
                        }
                        if !line_suffixes.is_empty() {
                            commands.push((indent.clone(), mode, doc));
                            for suffix in line_suffixes.iter().rev() {
                                commands.push((
                                    indent.clone(),
                                    mode,
                                    Doc::Command(DocCommand::LineSuffix(suffix)),
                                ));
                            }
                            line_suffixes.clear();
                        }
                        if line_mode == LineMode::HardLiteral {
                            out.push(OutKind::String(NEW_LINE.to_string()));
                            pos = 0;
                        } else {
                            trim(&mut out);
                            out.push(OutKind::String(NEW_LINE.to_string() + &indent.value));
                            pos = indent.length;
                        }
                    }
                }
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
