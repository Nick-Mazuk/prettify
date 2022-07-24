use crate::{add_integer_underscores, add_integer_underscores_reverse, Float};
use prettify::{concat, string, PrettifyDoc};

pub fn format_float(float: Float) -> PrettifyDoc {
    concat(vec![
        string(if float.is_negative { "-" } else { "" }),
        string(add_integer_underscores(float.integer)),
        string("."),
        match float.fraction {
            Some(fraction) => string(add_integer_underscores_reverse(fraction)),
            None => string("0"),
        },
        match float.exponent {
            Some(exponent) => concat(vec![
                string("e"),
                string(if float.exponent_is_negative { "-" } else { "" }),
                string(add_integer_underscores(exponent)),
            ]),
            None => string(""),
        },
    ])
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify::print;

    #[test]
    fn test_format_float() {
        assert_eq!(
            print(format_float(Float {
                is_negative: false,
                integer: "1",
                fraction: Some("0"),
                exponent_is_negative: false,
                exponent: None,
            })),
            "1.0"
        );
        assert_eq!(
            print(format_float(Float {
                is_negative: true,
                integer: "1",
                fraction: Some("0"),
                exponent_is_negative: false,
                exponent: None,
            })),
            "-1.0"
        );
        assert_eq!(
            print(format_float(Float {
                is_negative: false,
                integer: "1",
                fraction: None,
                exponent_is_negative: false,
                exponent: None,
            })),
            "1.0"
        );
        assert_eq!(
            print(format_float(Float {
                is_negative: false,
                integer: "1",
                fraction: Some("0"),
                exponent_is_negative: false,
                exponent: Some("2"),
            })),
            "1.0e2"
        );
        assert_eq!(
            print(format_float(Float {
                is_negative: false,
                integer: "1",
                fraction: Some("0"),
                exponent_is_negative: true,
                exponent: Some("2"),
            })),
            "1.0e-2"
        );

        assert_eq!(
            print(format_float(Float {
                is_negative: false,
                integer: "123456",
                fraction: Some("1234567"),
                exponent_is_negative: false,
                exponent: Some("12345678"),
            })),
            "123_456.123_456_7e12_345_678"
        );
    }
}
