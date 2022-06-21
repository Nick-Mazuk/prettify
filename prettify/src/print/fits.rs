use super::super::doc::{Doc, DocCommand, DocOptions, LineMode, PrettifyConfig};
use super::align::make_align;
use super::indent::make_indent;
use super::shared::{Command, Commands, Mode, Out, OutKind};
use super::trim::trim;
use std::borrow::{Borrow, Cow};

pub fn fits<'a>(
    next: &Command<'a>,
    rest_commands: &Commands<'a>,
    width: usize,
    doc_options: &DocOptions<'a>,
    has_line_suffix_default: bool,
    must_be_flat: bool,
    config: &PrettifyConfig,
) -> bool {
    let mut remainder = width;
    let mut remainder_index = rest_commands.len();
    let mut commands: Commands = vec![next.clone()];
    let mut out: Out = Vec::new();
    let mut has_line_suffix = has_line_suffix_default;
    while remainder > 0 {
        if commands.len() == 0 {
            if remainder_index == 0 {
                return true;
            }
            commands.push(rest_commands[remainder_index - 1].clone());
            remainder_index -= 1;
            continue;
        }
        let (indent, mode, doc) = commands.pop().unwrap();

        let borrowed_doc: &Doc = doc.borrow();

        match borrowed_doc.clone() {
            Doc::String(string) => {
                if string.len() > remainder {
                    return false;
                }
                out.push(OutKind::String(string.to_string()));
                remainder -= string.len();
            }
            Doc::Children(children) => {
                for child in children.into_iter().rev() {
                    commands.push((indent.clone(), mode, child.clone()));
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
                    remainder += trim(&mut out);
                }
                DocCommand::Group(contents, options) => {
                    if must_be_flat && options.should_break {
                        return false;
                    }
                    let group_mode = if options.should_break {
                        Mode::Break
                    } else {
                        mode
                    };
                    let new_contents = if !doc_options.expanded_states.is_empty()
                        && group_mode == Mode::Break
                    {
                        Cow::Owned(
                            (&options.expanded_states[options.expanded_states.len() - 1]).clone(),
                        )
                    } else {
                        *contents
                    };
                    commands.push((indent.clone(), group_mode, new_contents));
                }
                DocCommand::Fill(contents, _) => {
                    for child in contents.into_iter().rev() {
                        commands.push((indent.clone(), mode, child.clone()));
                    }
                }
                DocCommand::LineSuffix(_) => {
                    has_line_suffix = true;
                }
                DocCommand::LineSuffixBoundary => {
                    if has_line_suffix {
                        return false;
                    }
                }
                DocCommand::Line(line_mode) => match mode {
                    Mode::Break => {
                        return true;
                    }
                    Mode::Flat => match line_mode {
                        LineMode::Hard | LineMode::HardLiteral => {
                            return true;
                        }
                        LineMode::Auto => {
                            out.push(OutKind::String(" ".to_string()));
                            remainder -= 1;
                        }
                        LineMode::Soft => {}
                    },
                },
                DocCommand::BreakParent => {
                    return false;
                }
                DocCommand::IfBreak(_, _, _)
                | DocCommand::IndentIfBreak(_, _, _)
                | DocCommand::Root(_)
                | DocCommand::Cursor => {}
            },
        }
    }
    false
}
