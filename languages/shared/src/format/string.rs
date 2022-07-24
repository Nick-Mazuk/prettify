use prettify::{concat, string, PrettifyDoc};

fn count_single_quotes(s: &str) -> usize {
    s.matches('\'').count()
}

fn count_double_quotes(s: &str) -> usize {
    s.matches('\"').count()
}

fn convert_to_single_quoted_string(s: &str) -> String {
    let s = s.replace("\\\"", "\"");
    let mut result = String::new();
    let mut escaped = false;
    for char in s.chars() {
        if !escaped && char == '\\' {
            escaped = true;
        } else {
            if !escaped && char == '\'' {
                result.push('\\');
            }
            escaped = false;
        }
        result.push(char);
    }
    result
}

fn convert_to_double_quoted_string(s: &str) -> String {
    let s = s.replace("\\'", "'");
    let mut result = String::new();
    let mut escaped = false;
    for char in s.chars() {
        if !escaped && char == '\\' {
            escaped = true;
        } else {
            if !escaped && char == '"' {
                result.push('\\');
            }
            escaped = false;
        }
        result.push(char);
    }
    result
}

pub fn format_string_contents(s: &str) -> PrettifyDoc {
    if count_double_quotes(s) > count_single_quotes(s) {
        string(convert_to_single_quoted_string(s))
    } else {
        string(convert_to_double_quoted_string(s))
    }
}

pub fn format_string(s: &str) -> PrettifyDoc {
    if count_double_quotes(s) > count_single_quotes(s) {
        concat(vec![
            string("'"),
            string(convert_to_single_quoted_string(s)),
            string("'"),
        ])
    } else {
        concat(vec![
            string("\""),
            string(convert_to_double_quoted_string(s)),
            string("\""),
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify::print;

    #[test]
    fn double_quoted_strings() {
        assert_eq!(print(format_string("hello world")), "\"hello world\"");
        assert_eq!(print(format_string("y'all")), "\"y'all\"");
        assert_eq!(print(format_string("\" \" \\'''")), "\"\\\" \\\" '''\"");
        assert_eq!(print(format_string("")), "\"\"");
        assert_eq!(print(format_string("'")), "\"'\"");
        assert_eq!(print(format_string("\\'")), "\"'\"");
    }

    #[test]
    fn single_quoted_strings() {
        assert_eq!(print(format_string("\"")), "'\"'");
        assert_eq!(print(format_string("\\\"")), "'\"'");
        assert_eq!(print(format_string("\" \" '")), "'\" \" \\''");
        assert_eq!(print(format_string("\" \" \\'")), "'\" \" \\''");
    }
}
