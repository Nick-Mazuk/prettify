use super::shared::{Out, OutKind};
use regex::Regex;

pub fn trim(out: &mut Out) -> usize {
    if out.is_empty() {
        return 0;
    }

    let all_whitespace = Regex::new(r"^[\t ]*$").unwrap();
    let mut trim_count = 0;
    while !out.is_empty() {
        let final_item = &out[out.len() - 1];
        match final_item {
            OutKind::String(string) if all_whitespace.is_match(string) => {
                trim_count += string.len();
            }
            _ => break,
        }
        out.pop();
    }

    let trailing_whitespace = Regex::new(r"([\t ]*)$").unwrap();
    if !out.is_empty() {
        let mut matched = false;
        let mut string = String::new();
        match &out[out.len() - 1] {
            OutKind::String(str) if trailing_whitespace.is_match(str) => {
                matched = true;
                string = str.to_string();
            }
            _ => {}
        }
        if matched {
            trim_count += string.len();
            out.pop();
            out.push(OutKind::String(
                trailing_whitespace.replace(&string, "").to_string(),
            ));
        }
    }

    trim_count
}
