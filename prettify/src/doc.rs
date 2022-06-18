use std::borrow::Cow;

#[derive(PartialEq, Debug, Clone)]
pub enum LineMode {
    Hard,
    Soft,
    Auto,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AlignAmount {
    Spaces(usize),
    String(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum DocCommand<'a> {
    Group(Box<Cow<'a, Doc<'a>>>, Cow<'a, DocOptions<'a>>),
    // ConditionalGroup,
    Fill(Vec<Cow<'a, Doc<'a>>>, DocOptions<'a>),
    // IfBreak,
    // BreakParent,
    Line(LineMode),
    // SoftLine,
    // HardLine,
    // LiteralLine,
    LineSuffix(&'a str),
    LineSuffixBoundary,
    // Indent(contents)
    Indent(Box<Cow<'a, Doc<'a>>>),
    // Dedent,
    Align(Box<Cow<'a, Doc<'a>>>, AlignAmount),
    // MarkAsRoot,
    // DedentAsRoot,
    Trim,
    // IndentIfBreak,
}

#[derive(PartialEq, Debug, Clone, Copy)]
// or Doc Opt for short…
// …didn't see that Spider-man reference coming, did you?
pub struct DocOptions<'a> {
    pub should_break: bool,
    pub id: &'a str,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Doc<'a> {
    String(&'a str),
    Children(Vec<Cow<'a, Doc<'a>>>),
    Command(DocCommand<'a>),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct PrettifyConfig {
    pub tab_width: usize,
}
