use prettify::{concat, hard_line, string, PrettifyDoc};

pub fn format_atx_heading(size: usize, content: &str) -> PrettifyDoc {
    let heading_marker = "#";
    let heading_marker = heading_marker.repeat(size) + " ";

    concat(vec![string(heading_marker), string(content), hard_line()])
}

pub fn format_setext_heading(size: usize, content: &str) -> PrettifyDoc {
    if !content.contains('\n') {
        return format_atx_heading(size, content);
    }
    let heading_marker = if size == 1 { "=" } else { "-" };
    let heading_marker = heading_marker.repeat(10);

    concat(vec![
        string(content),
        hard_line(),
        string(heading_marker),
        hard_line(),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettify::print;

    #[test]
    fn format_atx_heading_test() {
        assert_eq!(
            print(format_atx_heading(1, "hello world")),
            "# hello world\n"
        );
    }

    #[test]
    fn format_setext_heading_test() {
        assert_eq!(
            print(format_setext_heading(1, "hello world")),
            "# hello world\n"
        );
        assert_eq!(
            print(format_setext_heading(2, "hello world")),
            "## hello world\n"
        );
        assert_eq!(
            print(format_setext_heading(1, "hello\nworld")),
            "hello\nworld\n==========\n"
        );
        assert_eq!(
            print(format_setext_heading(2, "hello\nworld")),
            "hello\nworld\n----------\n"
        );
    }
}
