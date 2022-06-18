use super::super::doc::{Doc, DocOptions};
use super::fits::fits;
use super::shared::{Command, Indent, LineSuffixes, Mode, PRINT_WIDTH};
use std::borrow::Cow;

// Fills each line with as much code as possible before moving to a new
// line with the same indentation.
//
// Expects doc.parts to be an array of alternating content and
// whitespace. The whitespace contains the line breaks.
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
pub fn process_fill<'a>(
    commands: &mut Vec<Command<'a>>,
    contents: Vec<Cow<'a, Doc<'a>>>,
    indent: &Indent,
    pos: &usize,
    line_suffixes: &LineSuffixes<'a>,
    doc_options: &DocOptions,
    mode: &Mode,
) {
    let remainder = PRINT_WIDTH - pos;
    if contents.is_empty() {
        return;
    }
    let content = &contents[0];
    let contents_command_flat: Command = (indent.clone(), Mode::Flat, content.clone());
    let contents_command_break: Command = (indent.clone(), Mode::Break, content.clone());
    let content_fits = fits(
        &contents_command_flat,
        &Vec::new(),
        remainder,
        doc_options,
        !line_suffixes.is_empty(),
        true,
    );
    if contents.len() == 1 {
        if content_fits {
            commands.push(contents_command_flat);
        } else {
            commands.push(contents_command_break);
        }
        return;
    }

    let whitespace = &contents[1];
    let whitespace_command_flat: Command = (indent.clone(), Mode::Flat, whitespace.clone());
    let whitespace_command_break: Command = (indent.clone(), Mode::Break, whitespace.clone());

    if contents.len() == 2 {
        if content_fits {
            commands.push(contents_command_flat);
            commands.push(whitespace_command_flat);
        } else {
            commands.push(contents_command_break);
            commands.push(whitespace_command_break);
        }
        return;
    }

    let mut cloned_contents = contents.clone();
    cloned_contents.remove(0);
    cloned_contents.remove(0);
    let remaining_command: Command = (
        indent.clone(),
        *mode,
        Cow::Owned(Doc::Children(cloned_contents.clone())),
    );
    let first_and_second_content_flat_command: Command = (
        indent.clone(),
        Mode::Flat,
        Cow::Owned(Doc::Children(cloned_contents)),
    );
    let first_and_second_content_fits = fits(
        &first_and_second_content_flat_command,
        &Vec::new(),
        remainder,
        doc_options,
        !line_suffixes.is_empty(),
        true,
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
