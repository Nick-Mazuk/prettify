use prettify::{print, string};

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
fn leading_whitespace() {
    assert_eq!(print(string(" string")), " string".to_string());
}

#[test]
fn trailing_whitespace() {
    assert_eq!(print(string("string ")), "string ".to_string());
}
