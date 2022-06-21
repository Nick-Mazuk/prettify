use prettify::{concat, group, literal_line, print, string};

#[test]
fn command_literal_line() {
    assert_eq!(print(literal_line()), "\n".to_string());
}

#[test]
fn command_literal_line_context() {
    assert_eq!(
        print(concat(vec![
            string("hello"),
            literal_line(),
            string("world")
        ])),
        "hello\nworld".to_string()
    );
}

#[test]
fn command_literal_line_context_inside_group() {
    assert_eq!(
        print(group(concat(vec![
            string("hello"),
            literal_line(),
            string("world")
        ]))),
        "hello\nworld".to_string()
    );
}

#[test]
fn command_literal_line_context_inside_group_long_text() {
    assert_eq!(
        print(group(concat(vec![
            string("this is a very long piece of text that definitely overflows the line"),
            literal_line(),
            string("this is a very long piece of text that definitely overflows the line")
        ]))),
        "this is a very long piece of text that definitely overflows the line\nthis is a very long piece of text that definitely overflows the line".to_string()
    );
}

#[test]
fn preserve_trailing_whitespace() {
    assert_eq!(
        print(group(concat(vec![
            string("hello    "),
            literal_line(),
            string("world")
        ]))),
        "hello    \nworld".to_string()
    );
}

#[test]
fn preserve_trailing_whitespace_long_text() {
    assert_eq!(
        print(group(concat(vec![
            string("this is a very long piece of text that definitely overflows the line    "),
            literal_line(),
            string("this is a very long piece of text that definitely overflows the line")
        ]))),
        "this is a very long piece of text that definitely overflows the line    \nthis is a very long piece of text that definitely overflows the line".to_string()
    );
}

#[test]
fn preserve_leading_whitespace() {
    assert_eq!(
        print(group(concat(vec![
            string("hello"),
            literal_line(),
            string("    world")
        ]))),
        "hello\n    world".to_string()
    );
}

#[test]
fn preserve_leading_whitespace_long_text() {
    assert_eq!(
        print(group(concat(vec![
            string("this is a very long piece of text that definitely overflows the line"),
            literal_line(),
            string("    this is a very long piece of text that definitely overflows the line")
        ]))),
        "this is a very long piece of text that definitely overflows the line\n    this is a very long piece of text that definitely overflows the line".to_string()
    );
}
