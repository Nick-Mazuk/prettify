use cli::{format_by_language, get_language_from_filename};

pub fn format_file(file_name: &str, contents: &str) -> String {
    let language = get_language_from_filename(file_name);
    match language {
        Some(matched_language) => format_by_language(contents, matched_language),
        _ => contents.to_string(),
    }
}

#[test]
fn test_formatting() {
    insta::glob!("files/**/*.*", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        let formatted = format_file(path.file_name().unwrap().to_str().unwrap(), &contents);
        insta::assert_snapshot!(formatted);
    });
}
