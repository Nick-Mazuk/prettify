use prettify::{break_parent, concat, group, if_break, print, string};

#[test]
fn if_break_break_state() {
    assert_eq!(
        print(group(concat(vec![
            string("hello "),
            if_break(string("world"), string("again"), "doc_id".to_string()),
            break_parent()
        ]))),
        "hello world".to_string()
    );
}

#[test]
fn if_break_flat_state() {
    assert_eq!(
        print(group(concat(vec![
            string("hello "),
            if_break(string("world"), string("again"), "doc_id".to_string()),
        ]))),
        "hello again".to_string()
    );
}
