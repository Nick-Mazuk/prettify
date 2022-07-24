use crate::{Integer, IntegerKind};
use prettify::{concat, string, PrettifyDoc};

pub fn format_integer(integer: Integer) -> PrettifyDoc {
    concat(vec![
        string(if integer.is_negative { "-" } else { "" }),
        match integer.kind {
            IntegerKind::Decimal => string(""),
            IntegerKind::Hexadecimal => string("0x"),
            IntegerKind::Octal => string("0o"),
            IntegerKind::Binary => string("0b"),
        },
        format_value(integer.value),
    ])
}

fn format_value(value: &str) -> PrettifyDoc {
    let value = value.to_ascii_lowercase();
    if value.len() <= 3 || value.contains("_") {
        string(value)
    } else {
        let mut new_value = String::new();
        for (index, char) in value.chars().rev().enumerate() {
            if index > 0 && index % 3 == 0 {
                new_value.push('_');
            }
            new_value.push(char);
        }
        string(new_value.chars().rev().collect::<String>())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify::print;

    #[test]
    fn test_format_integer() {
        assert_eq!(
            print(format_integer(Integer {
                is_negative: false,
                kind: IntegerKind::Decimal,
                value: "123_456",
            })),
            "123_456"
        );
        assert_eq!(
            print(format_integer(Integer {
                is_negative: true,
                kind: IntegerKind::Decimal,
                value: "1_23__67",
            })),
            "-1_23__67"
        );
        assert_eq!(
            print(format_integer(Integer {
                is_negative: false,
                kind: IntegerKind::Hexadecimal,
                value: "123_AbC",
            })),
            "0x123_abc"
        );
        assert_eq!(
            print(format_integer(Integer {
                is_negative: false,
                kind: IntegerKind::Octal,
                value: "123_456",
            })),
            "0o123_456"
        );
        assert_eq!(
            print(format_integer(Integer {
                is_negative: false,
                kind: IntegerKind::Binary,
                value: "10011_01",
            })),
            "0b10011_01"
        );

        assert_eq!(
            print(format_integer(Integer {
                is_negative: false,
                kind: IntegerKind::Decimal,
                value: "1234567",
            })),
            "1_234_567"
        );
        assert_eq!(
            print(format_integer(Integer {
                is_negative: false,
                kind: IntegerKind::Decimal,
                value: "12345678",
            })),
            "12_345_678"
        );
        assert_eq!(
            print(format_integer(Integer {
                is_negative: false,
                kind: IntegerKind::Decimal,
                value: "123456789",
            })),
            "123_456_789"
        );
    }
}
