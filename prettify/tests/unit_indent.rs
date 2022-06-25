use indoc::indoc;
use prettify::{concat, group, hard_line, if_break, indent, join, line, print, soft_line, string};

#[test]
fn indent_simple_string() {
    assert_eq!(
        print(group(indent(concat(vec![hard_line(), string("indented")])))),
        "\n    indented".to_string()
    );
}

#[test]
fn indent_array() {
    let mut items = Vec::new();
    for i in 0..12 {
        items.push(string(format!("item{}", i)));
    }
    assert_eq!(
        print(group(concat(vec![
            string("["),
            indent(concat(vec![
                soft_line(),
                join(items, concat(vec![string(","), line()])),
                if_break(string(","), string(""), String::from("comma")),
            ])),
            soft_line(),
            string("]"),
        ]))),
        indoc! {r#"
            [
                item0,
                item1,
                item2,
                item3,
                item4,
                item5,
                item6,
                item7,
                item8,
                item9,
                item10,
                item11,
            ]"#}
    );
}

#[test]
fn indents_array_only_when_wrapping_to_new_line() {
    let mut items = Vec::new();
    for i in 0..5 {
        items.push(string(format!("item{}", i)));
    }
    assert_eq!(
        print(group(concat(vec![
            string("["),
            indent(concat(vec![
                soft_line(),
                join(items, concat(vec![string(","), line()])),
                if_break(string(","), string(""), String::from("comma")),
            ])),
            soft_line(),
            string("]"),
        ]))),
        "[item0, item1, item2, item3, item4]".to_string()
    );
}

#[test]
fn nested_indents() {
    let mut items = Vec::new();
    for i in 0..12 {
        items.push(string(format!("item{}", i)));
    }
    items.push(concat(vec![
        string("["),
        indent(concat(vec![
            soft_line(),
            join(
                vec![string("itemA"), string("itemB")],
                concat(vec![string(","), line()]),
            ),
            if_break(string(","), string(""), String::from("comma")),
        ])),
        soft_line(),
        string("]"),
    ]));
    assert_eq!(
        print(group(concat(vec![
            string("["),
            indent(concat(vec![
                soft_line(),
                join(items, concat(vec![string(","), line()])),
                if_break(string(","), string(""), String::from("comma")),
            ])),
            soft_line(),
            string("]"),
        ]))),
        indoc! {r#"
            [
                item0,
                item1,
                item2,
                item3,
                item4,
                item5,
                item6,
                item7,
                item8,
                item9,
                item10,
                item11,
                [
                    itemA,
                    itemB,
                ],
            ]"#}
    );
}
