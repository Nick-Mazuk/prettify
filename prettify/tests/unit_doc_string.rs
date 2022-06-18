use prettify::{concat, print, string};

#[test]
fn doc_string_1() {
    assert_eq!(print(string("hello world")), "hello world".to_string());
}

#[test]
fn doc_string_2() {
    assert_eq!(
        print(string("another string")),
        "another string".to_string()
    );
}

#[test]
fn doc_children_string() {
    assert_eq!(
        print(concat(vec!(
            string("hello, "),
            string("world"),
            string("!")
        ))),
        "hello, world!".to_string()
    );
}
