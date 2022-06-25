use prettify::{concat, group, hard_line, line_suffix, print, string};

#[test]
fn line_suffix_command() {
    assert_eq!(
        print(group(concat(vec![
            string("a"),
            line_suffix(" // comment"),
            string(";"),
            hard_line()
        ]))),
        "a; // comment\n".to_string()
    );
}

#[test]
fn multiple_line_suffixes() {
    assert_eq!(
        print(group(concat(vec![
            string("a"),
            line_suffix(" //"),
            line_suffix(" "),
            line_suffix("comment"),
            string(";"),
            hard_line()
        ]))),
        "a; // comment\n".to_string()
    );
}

#[test]
fn separated_line_suffixes() {
    assert_eq!(
        print(group(concat(vec![
            line_suffix(" //"),
            string("a"),
            string(";"),
            line_suffix(" comment"),
            hard_line()
        ]))),
        "a; // comment\n".to_string()
    );
}
