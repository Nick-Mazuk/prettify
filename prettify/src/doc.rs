use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub enum LineMode {
    Hard,
    Soft,
    Auto,
    HardLiteral,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AlignAmount {
    Spaces(usize),
    String(String),
    DedentToRoot,
    Dedent,
}

pub type Contents<'a> = Rc<Doc<'a>>;

#[derive(PartialEq, Debug, Clone)]
pub enum DocCommand<'a> {
    Group(Contents<'a>, Rc<DocOptions<'a>>),
    Fill(Vec<Rc<Doc<'a>>>, Rc<DocOptions<'a>>),
    IfBreak(Contents<'a>, Contents<'a>, String),
    // IndentIfBreak(Contents, group_id, negate)
    IndentIfBreak(Contents<'a>, String, bool),
    BreakParent,
    Line(LineMode),
    LineSuffix(&'a str),
    LineSuffixBoundary,
    Indent(Contents<'a>),
    Align(Contents<'a>, AlignAmount),
    Root(Contents<'a>),
    Cursor,
    Trim,
}

#[derive(PartialEq, Debug, Clone)]
// or Doc Opt for short…
// …didn't see that Spider-man reference coming, did you?
pub struct DocOptions<'a> {
    pub should_break: bool,
    pub id: &'a str,
    pub expanded_states: Vec<Rc<Doc<'a>>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Doc<'a> {
    String(String),
    Children(Vec<Rc<Doc<'a>>>),
    Command(DocCommand<'a>),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct PrettifyConfig {
    pub tab_width: usize,
}
