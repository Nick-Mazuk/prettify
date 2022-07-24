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

fn parse_exponent(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, (sign, digits)) =
        preceded(tag_no_case("e"), tuple((opt_sign, is_a("0123456789_"))))(input)?;

    Ok((
        remainder,
        concat(vec![
            string("e"),
            sign,
            string(add_integer_underscores(trim_value(digits))),
        ]),
    ))
}

/**
 Parses floats. It may be more forgiving than the specs in many languages,
 but that allows for a better developer experience. Specifically, though,
 this float parser follows the JavaScript spec for integers.
*/
pub fn float(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, (sign, integer, _, fraction, exponent)) = alt((
        tuple((
            opt_sign,
            map(is_a("0123456789_"), |result| Some(result)),
            opt(tag(".")),
            opt(is_a("0123456789_")),
            opt(parse_exponent),
        )),
        tuple((
            opt_sign,
            opt(is_a("0123456789_")),
            opt(tag(".")),
            map(is_a("0123456789_"), |result| Some(result)),
            opt(parse_exponent),
        )),
    ))(input)?;

    Ok((
        remainder,
        concat(vec![
            sign,
            string(add_integer_underscores(trim_value(integer.unwrap_or("")))),
            string("."),
            string(add_integer_underscores_reverse(trim_value(
                fraction.unwrap_or(""),
            ))),
            exponent.unwrap_or(string("")),
        ]),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_errors, assert_formatted};

    #[test]
    fn float_test() {
        assert_formatted(float("1.0"), ("", "1.0"));
        assert_formatted(float("-1.0"), ("", "-1.0"));
        assert_formatted(float("1.0e2"), ("", "1.0e2"));
        assert_formatted(float("1.0E-2"), ("", "1.0e-2"));
        assert_formatted(float("1.0e+2"), ("", "1.0e2"));
        assert_formatted(float("1.0e2_"), ("", "1.0e2"));
        assert_formatted(float("1.0E-2_"), ("", "1.0e-2"));
        assert_formatted(float("1."), ("", "1.0"));
        assert_formatted(float("1.e2"), ("", "1.0e2"));
        assert_formatted(float(".7"), ("", "0.7"));
        assert_formatted(float(".7e-4"), ("", "0.7e-4"));
        assert_formatted(
            float("123456.1234567e12345678"),
            ("", "123_456.123_456_7e12_345_678"),
        );
    }

    #[test]
    fn float_failure_test() {
        assert_errors(float("e6"));
        assert_errors(float("."));
        assert_errors(float("-."));
        assert_errors(float("-.e6"));
        assert_errors(float(".e6"));
        assert_errors(float("+.e6"));
    }
}
