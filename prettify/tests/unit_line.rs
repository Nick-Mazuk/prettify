use prettify::{concat, group, line, print, string};

#[test]
fn command_line() {
    assert_eq!(print(line()), "\n".to_string());
}

#[test]
fn command_line_context() {
    assert_eq!(
        print(concat(vec![string("hello"), line(), string("world")])),
        "hello\nworld".to_string()
    );
}

#[test]
fn command_line_inside_group() {
    assert_eq!(
        print(group(concat(vec![
            string("hello"),
            line(),
            string("world")
        ]))),
        "hello world".to_string()
    );
}

#[test]
fn command_line_inside_group_long_text() {
    assert_eq!(
        print(group(concat(vec![
            string("this is a very long piece of text that definitely overflows the line"),
            line(),
            string("this is a very long piece of text that definitely overflows the line")
        ]))),
        "this is a very long piece of text that definitely overflows the line\nthis is a very long piece of text that definitely overflows the line".to_string()
    );
}
