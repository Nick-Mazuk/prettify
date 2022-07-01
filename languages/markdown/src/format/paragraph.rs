use prettify::{concat, fill, hard_line, join_to_vector, line, string, PrettifyDoc};

pub fn format_paragraph<'a>(content: &'a str) -> PrettifyDoc<'a> {
    concat(vec![
        fill(join_to_vector(
            content
                .replace('\n', " ")
                .split(' ')
                .map(|word| string(word))
                .collect(),
            line(),
        )),
        hard_line(),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettify::print;

    #[test]
    fn paragraph() {
        assert_eq!(print(format_paragraph("hello world")), "hello world\n");
        assert_eq!(print(format_paragraph("hello\nworld")), "hello world\n");
    }
}
