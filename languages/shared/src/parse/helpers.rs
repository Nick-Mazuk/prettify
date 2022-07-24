use nom::{branch::alt, bytes::complete::tag};

pub fn trim_value(input: &str) -> &str {
    let value = input
        .trim_end_matches('_')
        .trim_start_matches(|char| char == '_' || char == '0' || char == ' ');
    if value == "" {
        "0"
    } else {
        value
    }
}

pub fn sign(input: &str) -> nom::IResult<&str, &str> {
    alt((tag("-"), tag("+")))(input)
}

pub fn sign_is_positive(input: &str) -> bool {
    match input {
        "+" => true,
        "-" => false,
        _ => unreachable!(),
    }
}
pub fn optional_sign_is_positive(input: Option<&str>) -> bool {
    match input {
        Some("+") | None => true,
        Some("-") => false,
        _ => unreachable!(),
    }
}
