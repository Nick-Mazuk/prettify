use super::shared::{Mode, Out};
use crate::LineMode;

pub fn process_line(
    line_mode: LineMode,
    mode: &Mode,
    out: &mut Out,
    pos: &mut usize,
    should_remeasure: &mut bool,
) {
    match mode {
        Mode::Flat => match line_mode {
            LineMode::Auto => {
                out.push(String::from(" "));
                *pos += 1;
            }
            LineMode::Hard | LineMode::Soft => {
                *should_remeasure = true;
            }
        },
        Mode::Break => {}
    }
}
