use super::fits::fits;
use super::shared::{Command, Indent, Mode, PRINT_WIDTH};
use crate::{Doc, DocCommand, DocOptions};

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
    contents: &'a Vec<Doc<'a>>,
    indent: &Indent,
    pos: &usize,
    line_suffixes: &Vec<String>,
    doc_options: &DocOptions,
    mode: &Mode,
    doc: &'a mut Doc<'a>,
) {
    let remainder = PRINT_WIDTH - pos;
    if contents.len() == 0 {
        return ();
    }
    let content = &mut contents[0];
    let contents_command_flat = (indent.clone(), Mode::Flat, content);
    let contents_command_break = (indent.clone(), Mode::Break, content);
    let content_fits = fits(
        &contents_command_flat,
        &Vec::new(),
        remainder,
        doc_options,
        line_suffixes.len() > 0,
        true,
    );
    if contents.len() == 1 {
        if content_fits {
            commands.push(contents_command_flat);
        } else {
            commands.push(contents_command_break);
        }
        return ();
    }

    let whitespace = &mut contents[1];
    let whitespace_command_flat = (indent.clone(), Mode::Flat, whitespace);
    let whitespace_command_break = (indent.clone(), Mode::Break, whitespace);

    if contents.len() == 2 {
        if content_fits {
            commands.push(contents_command_flat);
            commands.push(whitespace_command_flat);
        } else {
            commands.push(contents_command_break);
            commands.push(whitespace_command_break);
        }
        return ();
    }

    // At this point we've handled the first pair (context, separator)
    // and will create a new fill doc for the rest of the content.
    // Ideally we wouldn't mutate the array here but copying all the
    // elements to a new array would make this algorithm quadratic,
    // which is unusable for large arrays (e.g. large texts in JSX).
    let original_contents_ref = match doc {
        Doc::Command(DocCommand::Fill(ref mut original_contents, _)) => Some(original_contents),
        _ => None,
    }
    .unwrap();
    original_contents_ref.remove(0);
    original_contents_ref.remove(0);
    let second_content = &mut original_contents_ref[0].clone();
    let mut remaining_command: Command = (indent.clone(), *mode, doc);
    let mut first_and_second_content_flat_command = (indent.clone(), Mode::Flat, second_content);
    let first_and_second_content_fits = fits(
        &first_and_second_content_flat_command,
        &Vec::new(),
        remainder,
        doc_options,
        line_suffixes.len() > 0,
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
