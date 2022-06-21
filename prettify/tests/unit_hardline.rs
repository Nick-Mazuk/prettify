use prettify::{concat, group, hard_line, print, string};

#[test]
fn command_hardline() {
    assert_eq!(print(hard_line()), "\n".to_string());
}

#[test]
fn command_hardline_context() {
    assert_eq!(
        print(concat(vec![string("hello"), hard_line(), string("world")])),
        "hello\nworld".to_string()
    );
}

#[test]
fn command_hardline_context_inside_group() {
    assert_eq!(
        print(group(concat(vec![
            string("hello"),
            hard_line(),
            string("world")
        ]))),
        "hello\nworld".to_string()
    );
}

#[test]
fn command_hard_line_inside_group_long_text() {
    assert_eq!(
        print(group(concat(vec![
            string("this is a very long piece of text that definitely overflows the line"),
            hard_line(),
            string("this is a very long piece of text that definitely overflows the line")
        ]))),
        "this is a very long piece of text that definitely overflows the line\nthis is a very long piece of text that definitely overflows the line".to_string()
    );
}
