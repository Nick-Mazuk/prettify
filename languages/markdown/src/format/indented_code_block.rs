use prettify::{concat, hard_line, join, string, PrettifyDoc};

fn get_backtick_count(content: &str) -> usize {
    let mut count = 0;
    for c in content.chars() {
        if c == '`' {
            count += 1;
        } else {
            break;
        }
    }
    count
}

pub fn format_indented_code_block(code: Vec<&str>) -> PrettifyDoc {
    let mut backtick_count = 3;
    for line in code.iter() {
        backtick_count = std::cmp::max(backtick_count, get_backtick_count(line.trim()) + 1);
    }
    let backticks = "`".repeat(backtick_count);
    concat(vec![
        string(backticks.clone()),
        hard_line(),
        join(code.into_iter().map(string).collect(), hard_line()),
        hard_line(),
        string(backticks),
        hard_line(),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettify::print;

    #[test]
    fn paragraph() {
        assert_eq!(
            print(format_indented_code_block(vec!["hello world"])),
            "```\nhello world\n```\n"
        );
        assert_eq!(
            print(format_indented_code_block(vec!["hello world   "])),
            "```\nhello world\n```\n"
        );
        assert_eq!(
            print(format_indented_code_block(vec!["  hello world"])),
            "```\n  hello world\n```\n"
        );
        assert_eq!(
            print(format_indented_code_block(vec!["hello", "world"])),
            "```\nhello\nworld\n```\n"
        );
        assert_eq!(
            print(format_indented_code_block(vec![
                "hello", "", "   ", "world"
            ])),
            "```\nhello\n\n\nworld\n```\n"
        );
        assert_eq!(
            print(format_indented_code_block(vec!["hello  ", "world   "])),
            "```\nhello\nworld\n```\n"
        );
        assert_eq!(
            print(format_indented_code_block(vec!["hello", "```", "world"])),
            "````\nhello\n```\nworld\n````\n"
        );
    }
}
