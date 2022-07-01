use prettify::{concat, hard_line, string, PrettifyDoc};

pub fn format_atx_heading(size: usize, content: &str) -> PrettifyDoc {
    let heading_marker = "#";
    let heading_marker = heading_marker.repeat(size) + " ";

    concat(vec![string(heading_marker), string(content), hard_line()])
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
}
