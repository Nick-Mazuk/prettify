use prettify::{concat, dedent, group, hard_line, indent, print, string};

#[test]
fn dedent_from_root_does_nothing() {
    assert_eq!(print(group(dedent(string("hello")))), "hello".to_string());
}

#[test]
fn undoes_an_indent() {
    assert_eq!(
        print(group(indent(dedent(concat(vec![
            hard_line(),
            string("dedent")
        ]))))),
        "\ndedent".to_string()
    );
}
