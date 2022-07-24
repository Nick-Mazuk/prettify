use nom::{branch::alt, bytes::complete::tag, combinator::opt};
use prettify::{string, PrettifyDoc};

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

pub fn sign(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, char) = alt((tag("-"), tag("+")))(input)?;
    Ok((
        remainder,
        if char == "-" {
            string("-")
        } else {
            string("+")
        },
    ))
}

pub fn opt_sign(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, char) = opt(alt((tag("-"), tag("+"))))(input)?;
    Ok((
        remainder,
        match char {
            Some("+") | None => string(""),
            Some("-") => string("-"),
            _ => unreachable!(),
        },
    ))
}

pub fn add_integer_underscores_every_n(value: &str, n: usize) -> String {
    if value.len() <= n || value.contains("_") {
        value.to_string()
    } else {
        let mut new_value = String::new();
        for (index, char) in value.chars().rev().enumerate() {
            if index > 0 && index % n == 0 {
                new_value.push('_');
            }
            new_value.push(char);
        }
        new_value.chars().rev().collect::<String>()
    }
}

pub fn add_integer_underscores_reverse_every_n(value: &str, n: usize) -> String {
    if value.len() <= n || value.contains("_") {
        value.to_string()
    } else {
        let mut new_value = String::new();
        for (index, char) in value.chars().enumerate() {
            if index > 0 && index % n == 0 {
                new_value.push('_');
            }
            new_value.push(char);
        }
        new_value.chars().collect::<String>()
    }
}

pub fn add_integer_underscores(value: &str) -> String {
    add_integer_underscores_every_n(value, 3)
}

pub fn add_integer_underscores_reverse(value: &str) -> String {
    add_integer_underscores_reverse_every_n(value, 3)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_integer_underscores_every_n_test() {
        assert_eq!(
            add_integer_underscores_every_n("1234567890", 5),
            "12345_67890"
        );
        assert_eq!(
            add_integer_underscores_every_n("1234567890", 4),
            "12_3456_7890"
        );
        assert_eq!(
            add_integer_underscores_every_n("1234567890", 3),
            "1_234_567_890"
        );
        assert_eq!(
            add_integer_underscores_every_n("1234567890", 2),
            "12_34_56_78_90"
        );
        assert_eq!(
            add_integer_underscores_every_n("1234567890", 1),
            "1_2_3_4_5_6_7_8_9_0"
        );
    }

    #[test]
    fn add_integer_underscores_reverse_every_n_test() {
        assert_eq!(
            add_integer_underscores_reverse_every_n("1234567890", 5),
            "12345_67890"
        );
        assert_eq!(
            add_integer_underscores_reverse_every_n("1234567890", 4),
            "1234_5678_90"
        );
        assert_eq!(
            add_integer_underscores_reverse_every_n("1234567890", 3),
            "123_456_789_0"
        );
        assert_eq!(
            add_integer_underscores_reverse_every_n("1234567890", 2),
            "12_34_56_78_90"
        );
        assert_eq!(
            add_integer_underscores_reverse_every_n("1234567890", 1),
            "1_2_3_4_5_6_7_8_9_0"
        );
    }

    #[test]
    fn add_integer_underscores_test() {
        assert_eq!(add_integer_underscores("12345"), "12_345");
        assert_eq!(add_integer_underscores("1234567"), "1_234_567");
        assert_eq!(add_integer_underscores("12345678"), "12_345_678");
        assert_eq!(add_integer_underscores("123456789"), "123_456_789");
        assert_eq!(add_integer_underscores("1_2345"), "1_2345");
    }

    #[test]
    fn add_integer_underscores_reverse_test() {
        assert_eq!(add_integer_underscores_reverse("12345"), "123_45");
        assert_eq!(add_integer_underscores_reverse("1234567"), "123_456_7");
        assert_eq!(add_integer_underscores_reverse("12345678"), "123_456_78");
        assert_eq!(add_integer_underscores_reverse("123456789"), "123_456_789");
        assert_eq!(add_integer_underscores_reverse("1_2345"), "1_2345");
    }
}
