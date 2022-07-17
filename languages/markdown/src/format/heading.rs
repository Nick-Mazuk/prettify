use prettify::{
    concat, conditional_group, fill, hard_line, join, join_to_vector, line, string, PrettifyDoc,
};

fn atx_heading_marker(level: usize) -> PrettifyDoc<'static> {
    let heading_marker = "#";
    let heading_marker = heading_marker.repeat(level) + " ";
    string(heading_marker)
}

pub fn format_atx_heading(level: usize, content: &str) -> PrettifyDoc {
    concat(vec![
        atx_heading_marker(level),
        string(content),
        hard_line(),
    ])
}

pub fn format_setext_heading(level: usize, content: &str) -> PrettifyDoc {
    let heading_marker = if level == 1 { "=" } else { "-" };
    let heading_marker = heading_marker.repeat(12);

    let content_words: Vec<PrettifyDoc> =
        content.replace('\n', " ").split(' ').map(string).collect();

    concat(vec![
        conditional_group(
            vec![
                concat(vec![
                    atx_heading_marker(level),
                    join(content_words.clone(), line()),
                ]),
                concat(vec![
                    fill(join_to_vector(content_words, line())),
                    hard_line(),
                    string(heading_marker),
                ]),
            ],
            "setext_heading",
        ),
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
        assert_eq!(
            print(format_atx_heading(2, "hello world")),
            "## hello world\n"
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
            "# hello world\n"
        );
        assert_eq!(
            print(format_setext_heading(2, "hello\nworld")),
            "## hello world\n"
        );
        assert_eq!(print(format_setext_heading(2, "hello")), "## hello\n");
        assert_eq!(
            print(format_setext_heading(
                1,
                "this is an incredibly long header that definitely cannot fit on one line so this will need to be rendered as a setext heading"
            )),
            "this is an incredibly long header that definitely cannot fit on one line so this\nwill need to be rendered as a setext heading\n============\n"
        );
    }
}
