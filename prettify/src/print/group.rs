use super::super::doc::{Doc, DocOptions, PrettifyConfig};
use super::fits::fits;
use super::shared::{Command, Indent, LineSuffixes, Mode, PRINT_WIDTH};
use std::borrow::Cow;
use std::collections::HashMap;

pub fn process_group<'a>(
    contents: Cow<'a, Doc<'a>>,
    doc_options: Cow<'a, DocOptions<'a>>,
    commands: &mut Vec<Command<'a>>,
    line_suffixes: &mut LineSuffixes<'a>,
    indent: Indent,
    mode: &Mode,
    group_mode_map: &mut HashMap<&'a str, Mode>,
    pos: &mut usize,
    should_remeasure: &mut bool,
    config: &PrettifyConfig,
) {
    if *mode == Mode::Flat && !*should_remeasure {
        commands.push((
            indent.clone(),
            if doc_options.should_break {
                Mode::Break
            } else {
                Mode::Flat
            },
            contents.clone(),
        ));
    };
    *should_remeasure = false;
    let mut next_mode = Mode::Flat;
    // "&*" unboxes the contents and creates a reference of it
    let next: Command = (indent.clone(), next_mode, contents.clone());
    let remainder = PRINT_WIDTH - *pos;
    let has_line_suffix = !line_suffixes.is_empty();
    if !doc_options.should_break
        && fits(
            &next,
            commands,
            remainder,
            &doc_options,
            has_line_suffix,
            false,
            config,
        )
    {
        commands.push(next);
    } else {
        *should_remeasure = true;
        next_mode = Mode::Break;
        commands.push((indent, Mode::Break, contents));
    }
    group_mode_map.insert(doc_options.id, next_mode);
}
