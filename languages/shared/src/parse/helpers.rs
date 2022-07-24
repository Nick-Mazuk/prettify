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
