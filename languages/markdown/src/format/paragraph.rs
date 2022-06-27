use prettify::{concat, hard_line, string, PrettifyDoc};

pub fn format_paragraph<'a>(content: &'a str) -> PrettifyDoc<'a> {
    concat(vec![string(content), hard_line()])
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettify::print;

    #[test]
    fn header_1() {
        assert_eq!(print(format_paragraph("hello world")), "hello world\n");
    }
}
