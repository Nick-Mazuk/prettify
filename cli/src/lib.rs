use prettify::print;
use prettify_markdown::format_markdown;
use prettify_toml::format_toml;
use std::time::Duration;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Language {
    Markdown,
    Toml,
}

pub fn format_by_language(contents: &str, language: Language) -> String {
    let doc = match language {
        Language::Markdown => format_markdown(contents),
        Language::Toml => format_toml(contents),
    };
    match doc {
        Ok(doc) => print(doc),
        Err(_) => contents.to_string(),
    }
}

pub fn get_language_from_filename(filename: &str) -> Option<Language> {
    if filename.ends_with(".md") {
        Some(Language::Markdown)
    } else if filename.ends_with(".toml") {
        Some(Language::Toml)
    } else {
        None
    }
}

pub fn get_elapsed_string(elapsed: Duration) -> String {
    if elapsed.as_millis() < 1 {
        format!(
            "{} microsecond{}",
            elapsed.as_micros(),
            if elapsed.as_micros() == 1 { "" } else { "s" }
        )
    } else if elapsed.as_secs() < 1 {
        format!(
            "{} millisecond{}",
            elapsed.as_millis(),
            if elapsed.as_millis() == 1 { "" } else { "s" }
        )
    } else {
        format!(
            "{} second{}",
            elapsed.as_secs(),
            if elapsed.as_secs() == 1 { "" } else { "s" }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_elapsed_string_test() {
        assert_eq!(
            get_elapsed_string(Duration::from_micros(1)),
            "1 microsecond"
        );
        assert_eq!(
            get_elapsed_string(Duration::from_micros(2)),
            "2 microseconds"
        );
        assert_eq!(
            get_elapsed_string(Duration::from_millis(1)),
            "1 millisecond"
        );
        assert_eq!(
            get_elapsed_string(Duration::from_millis(2)),
            "2 milliseconds"
        );
        assert_eq!(get_elapsed_string(Duration::from_millis(1000)), "1 second");
        assert_eq!(get_elapsed_string(Duration::from_millis(1111)), "1 second");
        assert_eq!(get_elapsed_string(Duration::from_millis(2000)), "2 seconds");
    }
}
