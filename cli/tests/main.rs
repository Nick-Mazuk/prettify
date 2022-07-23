use prettify_cli::{format_by_language, get_language_from_filename};

pub fn format_file(file_name: &str, contents: &str) -> String {
    let language = get_language_from_filename(file_name);
    match language {
        Some(matched_language) => format_by_language(contents, matched_language),
        _ => panic!("{} is not a supported file type", file_name),
    }
}

#[test]
fn test_formatting() {
    let header = indoc::indoc! {r#"
    |                                                                              | printWidth
    ----------------------------------start input-----------------------------------
    "#};
    let footer = indoc::indoc! {r#"
    -----------------------------------end input------------------------------------
    |                                                                              | printWidth
    "#};
    insta::glob!("files/**/*.*", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        let formatted = format_file(path.file_name().unwrap().to_str().unwrap(), &contents);
        let file = format!("{}{}\n{}", header, formatted, footer);
        insta::assert_snapshot!(file);
    });
}

#[test]
fn test_idempotency() {
    // this test ensures that the formatting an already formatted document doesn't
    // change the formatting.
    insta::glob!("files/**/*.*", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let formatted = format_file(file_name, &contents);
        assert_eq!(formatted, format_file(file_name, &formatted));
    });
}
