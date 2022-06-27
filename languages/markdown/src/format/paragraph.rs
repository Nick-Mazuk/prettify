use prettify::{concat, fill, hard_line, join_to_vector, line, string, PrettifyDoc};

pub fn format_paragraph<'a>(content: &'a str) -> PrettifyDoc<'a> {
    concat(vec![
        fill(join_to_vector(
            content.split(' ').map(|word| string(word)).collect(),
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
    fn header_1() {
        assert_eq!(print(format_paragraph("hello world")), "hello world\n");
    }
}
