use prettify::{join, print, string};

#[test]
fn join_zero_elements() {
    assert_eq!(print(join(vec!(), string(","))), "".to_string());
}

#[test]
fn join_one_element() {
    assert_eq!(print(join(vec!(string("a")), string(","))), "a".to_string());
}

#[test]
fn join_two_elements() {
    assert_eq!(
        print(join(vec!(string("a"), string("b")), string(","))),
        "a,b".to_string()
    );
}

#[test]
fn join_three_elements() {
    assert_eq!(
        print(join(
            vec!(string("a"), string("b"), string("c")),
            string(",")
        )),
        "a,b,c".to_string()
    );
}

#[test]
fn leading_whitespace() {
    assert_eq!(
        print(join(
            vec!(string("a"), string("b"), string("c")),
            string(" ,")
        )),
        "a ,b ,c".to_string()
    );
}

#[test]
fn trailing_whitespace() {
    assert_eq!(
        print(join(
            vec!(string("a"), string("b"), string("c")),
            string(", ")
        )),
        "a, b, c".to_string()
    );
}

#[test]
fn join_custom_separator() {
    assert_eq!(
        print(join(vec!(string("a"), string("b")), string("•"))),
        "a•b".to_string()
    );
}
