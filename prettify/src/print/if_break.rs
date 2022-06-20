use super::super::doc::Contents;
use super::shared::{Commands, GroupModeMap, Indent, Mode};
use crate::indent as build_indent;
use std::borrow::Cow;

pub fn process_if_break<'a>(
    break_contents: Contents<'a>,
    flat_contents: Contents<'a>,
    group_id: String,
    mode: &Mode,
    group_mode_map: &GroupModeMap<'a>,
    indent: Indent,
    commands: &mut Commands<'a>,
) {
    let group_mode = match group_mode_map.get(&group_id as &str) {
        Some(mode) => mode,
        None => mode,
    };
    match group_mode {
        Mode::Break => {
            commands.push((indent, *mode, *break_contents));
        }
        Mode::Flat => {
            commands.push((indent, *mode, *flat_contents));
        }
    }
}

pub fn process_indent_if_break<'a>(
    contents: Contents<'a>,
    group_id: String,
    negate: bool,
    mode: &Mode,
    group_mode_map: &GroupModeMap<'a>,
    indent: Indent,
    commands: &mut Commands<'a>,
) {
    let group_mode = match group_mode_map.get(&group_id as &str) {
        Some(mode) => mode,
        None => mode,
    };
    match group_mode {
        Mode::Break => {
            if negate {
                commands.push((indent, *mode, *contents));
            } else {
                commands.push((
                    indent,
                    *mode,
                    Cow::Owned(build_indent(contents.into_owned())),
                ));
            }
        }
        Mode::Flat => {
            if negate {
                commands.push((
                    indent,
                    *mode,
                    Cow::Owned(build_indent(contents.into_owned())),
                ));
            } else {
                commands.push((indent, *mode, *contents));
            }
        }
    }
}
