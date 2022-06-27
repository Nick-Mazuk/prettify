use prettify_cli::{format_by_language, get_language_from_filename};

pub fn format_file(file_name: &str, contents: &str) -> String {
    let language = get_language_from_filename(file_name);
    match language {
        Some(matched_language) => format_by_language(contents, matched_language),
        _ => contents.to_string(),
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
