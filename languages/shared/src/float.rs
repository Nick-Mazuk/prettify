use crate::helpers::{
    add_integer_underscores, add_integer_underscores_reverse, opt_sign, trim_value,
};
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, tag_no_case},
    combinator::{map, opt},
    sequence::{preceded, tuple},
};
use prettify::{concat, string, PrettifyDoc};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FloatOptions {
    use_underscores: bool,
}

impl FloatOptions {
    pub fn new() -> Self {
        FloatOptions {
            use_underscores: false,
        }
    }

    pub fn use_underscores(mut self) -> Self {
        self.use_underscores = true;
        self
    }
}

fn parse_exponent<'a>(
    options: FloatOptions,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    map(
        preceded(tag_no_case("e"), tuple((opt_sign, is_a("0123456789_")))),
        move |(sign, mut digits)| {
            digits = trim_value(digits);
            concat(vec![
                string("e"),
                sign,
                string(if options.use_underscores {
                    add_integer_underscores(digits)
                } else {
                    digits.replace('_', "")
                }),
            ])
        },
    )
}

/**
 Parses floats. It may be more forgiving than the specs in many languages,
 but that allows for a better developer experience. Specifically, though,
 this float parser follows the JavaScript spec for integers.
*/
pub fn float<'a>(
    options: FloatOptions,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    map(
        alt((
            tuple((
                opt_sign,
                map(is_a("0123456789_"), Some),
                opt(tag(".")),
                opt(is_a("0123456789_")),
                map(parse_exponent(options), Some),
            )),
            tuple((
                opt_sign,
                map(is_a("0123456789_"), Some),
                map(tag("."), Some),
                opt(is_a("0123456789_")),
                opt(parse_exponent(options)),
            )),
            tuple((
                opt_sign,
                opt(is_a("0123456789_")),
                map(tag("."), Some),
                map(is_a("0123456789_"), Some),
                opt(parse_exponent(options)),
            )),
        )),
        move |(sign, integer, _, fraction, exponent)| {
            concat(vec![
                sign,
                string(if options.use_underscores {
                    add_integer_underscores(trim_value(integer.unwrap_or("")))
                } else {
                    trim_value(integer.unwrap_or("")).replace('_', "")
                }),
                string("."),
                string(if options.use_underscores {
                    add_integer_underscores_reverse(trim_value(fraction.unwrap_or("")))
                } else {
                    trim_value(fraction.unwrap_or("")).replace('_', "")
                }),
                exponent.unwrap_or_else(|| string("")),
            ])
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_errors, assert_formatted};

    #[test]
    fn float_test() {
        let options = FloatOptions::new().use_underscores();
        assert_formatted(float(options)("1.0"), ("", "1.0"));
        assert_formatted(float(options)("-1.0"), ("", "-1.0"));
        assert_formatted(float(options)("1.0e2"), ("", "1.0e2"));
        assert_formatted(float(options)("1.0E-2"), ("", "1.0e-2"));
        assert_formatted(float(options)("1.0e+2"), ("", "1.0e2"));
        assert_formatted(float(options)("1.0e2_"), ("", "1.0e2"));
        assert_formatted(float(options)("1.0E-2_"), ("", "1.0e-2"));
        assert_formatted(float(options)("1."), ("", "1.0"));
        assert_formatted(float(options)("1.e2"), ("", "1.0e2"));
        assert_formatted(float(options)(".7"), ("", "0.7"));
        assert_formatted(float(options)(".7e-4"), ("", "0.7e-4"));
        assert_formatted(float(options)("0000001.000000000e0000010"), ("", "1.0e10"));
        assert_formatted(
            float(options)("123456.1234567e12345678"),
            ("", "123_456.123_456_7e12_345_678"),
        );
    }

    #[test]
    fn no_underscores() {
        assert_formatted(
            float(FloatOptions::new())("123456.1234567e12345678"),
            ("", "123456.1234567e12345678"),
        );
        assert_formatted(
            float(FloatOptions::new())("123_45_6.123456_7e12_345678"),
            ("", "123456.1234567e12345678"),
        );
    }

    #[test]
    fn float_failure_test() {
        let options = FloatOptions::new();
        assert_errors(float(options)("e6"));
        assert_errors(float(options)("."));
        assert_errors(float(options)("-."));
        assert_errors(float(options)("-.e6"));
        assert_errors(float(options)(".e6"));
        assert_errors(float(options)("+.e6"));
        assert_errors(float(options)("123"));
    }
}
