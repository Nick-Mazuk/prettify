use super::super::doc::{Doc, DocCommand, LineMode};
use super::break_parent::break_parent;
use super::concat::concat;
use std::rc::Rc;

pub fn line<'a>() -> Rc<Doc<'a>> {
    Rc::new(Doc::Command(DocCommand::Line(LineMode::Auto)))
}

pub fn soft_line<'a>() -> Rc<Doc<'a>> {
    Rc::new(Doc::Command(DocCommand::Line(LineMode::Soft)))
}

pub fn hard_line_without_break_parent<'a>() -> Rc<Doc<'a>> {
    Rc::new(Doc::Command(DocCommand::Line(LineMode::Hard)))
}

pub fn literal_line_without_break_parent<'a>() -> Rc<Doc<'a>> {
    Rc::new(Doc::Command(DocCommand::Line(LineMode::HardLiteral)))
}

pub fn hard_line<'a>() -> Rc<Doc<'a>> {
    concat(vec![hard_line_without_break_parent(), break_parent()])
}

pub fn literal_line<'a>() -> Rc<Doc<'a>> {
    concat(vec![literal_line_without_break_parent(), break_parent()])
}
