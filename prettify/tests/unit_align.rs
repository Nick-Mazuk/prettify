use prettify::{align, concat, group, hard_line, print, string, AlignAmount};

#[test]
fn space_align() {
    assert_eq!(
        print(group(align(
            concat(vec![hard_line(), string("aligned")]),
            AlignAmount::Spaces(4)
        ))),
        "\n    aligned".to_string()
    );
    assert_eq!(
        print(group(align(
            concat(vec![hard_line(), string("aligned")]),
            AlignAmount::Spaces(8)
        ))),
        "\n        aligned".to_string()
    );
}

#[test]
fn string_align() {
    assert_eq!(
        print(group(align(
            concat(vec![hard_line(), string("aligned")]),
            AlignAmount::String("----".to_string())
        ))),
        "\n----aligned".to_string()
    );
}
