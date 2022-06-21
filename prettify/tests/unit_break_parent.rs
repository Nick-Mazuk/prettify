use prettify::{break_parent, concat, group, line, print, string};

#[test]
fn break_parent_command() {
    assert_eq!(
        print(group(concat(vec![
            string("hello"),
            line(),
            string("world"),
            break_parent()
        ]))),
        "hello\nworld".to_string()
    );
}

#[test]
fn nested_groups() {
    assert_eq!(
        print(group(concat(vec![
            string("hello"),
            line(),
            string("world"),
            group(concat(vec![
                string(","),
                line(),
                string("again"),
                break_parent()
            ]))
        ]))),
        "hello\nworld,\nagain".to_string()
    );
}

#[test]
fn nested_groups_only_outer() {
    assert_eq!(
        print(group(concat(vec![
            string("hello"),
            line(),
            string("world"),
            break_parent(),
            group(concat(vec![string(","), line(), string("again"),]))
        ]))),
        "hello\nworld, again".to_string()
    );
}
