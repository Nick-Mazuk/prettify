use prettify::print;
use prettify_markdown::format_markdown;
use std::time::Duration;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Language {
    Markdown,
}

pub fn format_by_language(contents: &str, language: Language) -> String {
    print(match language {
        Language::Markdown => format_markdown(contents),
    })
}

pub fn get_language_from_filename(filename: &str) -> Option<Language> {
    if filename.ends_with(".md") {
        Some(Language::Markdown)
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
