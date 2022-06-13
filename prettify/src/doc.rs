#[derive(PartialEq, Debug, Clone)]
pub enum DocCommand<'a> {
    Group(Box<Doc<'a>>),
    // ConditionalGroup,
    // Fill,
    // IfBreak,
    // BreakParent,
    Join(Box<Doc<'a>>, Vec<Doc<'a>>),
    // Line,
    // SoftLine,
    HardLine,
    // LiteralLine,
    // LineSuffix,
    // LineSuffixBoundary,
    // Indent,
    // Dedent,
    // Align,
    // MarkAsRoot,
    // DedentAsRoot,
    // Trim,
    // IndentIfBreak,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Doc<'a> {
    String(&'a str),
    Children(Vec<Doc<'a>>),
    Command(DocCommand<'a>),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct PrettifyConfig {
    pub tab_width: usize,
}
