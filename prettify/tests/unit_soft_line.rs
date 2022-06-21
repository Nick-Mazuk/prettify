use prettify::{concat, group, print, soft_line, string};

#[test]
fn command_soft_line() {
    assert_eq!(print(soft_line()), "\n".to_string());
}

#[test]
fn command_soft_line_context() {
    assert_eq!(
        print(concat(vec![string("hello"), soft_line(), string("world")])),
        "hello\nworld".to_string()
    );
}

#[test]
fn command_soft_line_inside_group() {
    assert_eq!(
        print(group(concat(vec![
            string("hello "),
            soft_line(),
            string("world")
        ]))),
        "hello world".to_string()
    );
}
