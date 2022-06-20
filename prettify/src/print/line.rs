use super::super::doc::{Doc, DocCommand, LineMode};
use super::shared::{Command, Indent, LineSuffixes, Mode, Out, NEW_LINE};
use super::trim::trim;
use std::borrow::Cow;

// https://sourcegraph.com/github.com/prettier/prettier/-/blob/src/document/doc-printer.js?L524
pub fn process_line<'a>(
    line_mode: LineMode,
    mode: &Mode,
    out: &mut Out,
    pos: &mut usize,
    should_remeasure: &mut bool,
    line_suffixes: &mut LineSuffixes<'a>,
    commands: &mut Vec<Command<'a>>,
    indent: Indent,
    doc: Cow<'a, Doc<'a>>,
) {
    if *mode == Mode::Flat && line_mode == LineMode::Auto {
        out.push(String::from(" "));
        *pos += 1;
        return;
    }
    if line_mode == LineMode::Hard || line_mode == LineMode::HardLiteral {
        *should_remeasure = true;
    }
    if !line_suffixes.is_empty() {
        commands.push((indent.clone(), mode.clone(), doc));
        for suffix in line_suffixes.into_iter().rev() {
            commands.push((
                indent.clone(),
                mode.clone(),
                Cow::Owned(Doc::Command(DocCommand::LineSuffix(suffix))),
            ));
        }
        line_suffixes.clear();
    }
    if line_mode == LineMode::HardLiteral {
        out.push(NEW_LINE.to_string());
        *pos = 0;
    } else {
        *pos -= trim(out);
        out.push(NEW_LINE.to_string() + &indent.value);
        *pos += indent.length;
    }
}
