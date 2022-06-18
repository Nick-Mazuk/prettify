use prettify::{concat, print, string};

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
