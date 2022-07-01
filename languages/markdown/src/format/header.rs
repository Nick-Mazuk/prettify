use prettify::{concat, hard_line, string, PrettifyDoc};

pub fn format_header(size: usize, content: &str) -> PrettifyDoc {
    let header_marker = "#";
    let header_marker = header_marker.repeat(size) + " ";

    concat(vec![string(header_marker), string(content), hard_line()])
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettify::print;

    #[test]
    fn header_1() {
        assert_eq!(print(format_header(1, "hello world")), "# hello world\n");
    }
}
