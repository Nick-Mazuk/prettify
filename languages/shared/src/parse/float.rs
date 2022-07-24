use super::helpers::{sign, trim_value};
use crate::Float;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, tag_no_case},
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

fn parse_exponent(input: &str) -> nom::IResult<&str, (Option<&str>, &str)> {
    preceded(tag_no_case("e"), tuple((opt(sign), is_a("0123456789_"))))(input)
}

/**
 Parses floats. It may be more forgiving than the specs in many languages,
 but that allows for a better developer experience. Specifically, though,
 this float parser follows the JavaScript spec for integers.
*/
pub fn float(input: &str) -> nom::IResult<&str, Float> {
    let (remainder, (sign, integer, _, fraction, exponent)) = alt((
        tuple((
            opt(sign),
            map(is_a("0123456789_"), |result| Some(result)),
            opt(tag(".")),
            opt(is_a("0123456789_")),
            opt(parse_exponent),
        )),
        tuple((
            opt(sign),
            opt(is_a("0123456789_")),
            opt(tag(".")),
            map(is_a("0123456789_"), |result| Some(result)),
            opt(parse_exponent),
        )),
    ))(input)?;

    Ok((
        remainder,
        Float {
            is_negative: match sign {
                Some("-") => true,
                _ => false,
            },
            integer: match integer {
                Some(integer) => trim_value(integer),
                None => "0",
            },
            fraction: match fraction {
                Some(fraction) => Some(trim_value(fraction)),
                None => None,
            },
            exponent_is_negative: match exponent {
                Some((Some(exponent), _)) => exponent == "-",
                _ => false,
            },
            exponent: match exponent {
                Some((_, value)) => Some(trim_value(value)),
                _ => None,
            },
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::assert_errors;

    #[test]
    fn float_test() {
        assert_eq!(
            float("1.0"),
            Ok((
                "",
                Float {
                    is_negative: false,
                    integer: "1",
                    fraction: Some("0"),
                    exponent_is_negative: false,
                    exponent: None,
                }
            ))
        );
        assert_eq!(
            float("-1.0"),
            Ok((
                "",
                Float {
                    is_negative: true,
                    integer: "1",
                    fraction: Some("0"),
                    exponent_is_negative: false,
                    exponent: None,
                }
            ))
        );
        assert_eq!(
            float("1.0e2"),
            Ok((
                "",
                Float {
                    is_negative: false,
                    integer: "1",
                    fraction: Some("0"),
                    exponent_is_negative: false,
                    exponent: Some("2"),
                }
            ))
        );
        assert_eq!(
            float("1.0e-2"),
            Ok((
                "",
                Float {
                    is_negative: false,
                    integer: "1",
                    fraction: Some("0"),
                    exponent_is_negative: true,
                    exponent: Some("2"),
                }
            ))
        );
        assert_eq!(
            float("1.0e+2"),
            Ok((
                "",
                Float {
                    is_negative: false,
                    integer: "1",
                    fraction: Some("0"),
                    exponent_is_negative: false,
                    exponent: Some("2"),
                }
            ))
        );
        assert_eq!(
            float("1.0e2_"),
            Ok((
                "",
                Float {
                    is_negative: false,
                    integer: "1",
                    fraction: Some("0"),
                    exponent_is_negative: false,
                    exponent: Some("2"),
                }
            ))
        );
        assert_eq!(
            float("1.0e-2_"),
            Ok((
                "",
                Float {
                    is_negative: false,
                    integer: "1",
                    fraction: Some("0"),
                    exponent_is_negative: true,
                    exponent: Some("2"),
                }
            ))
        );
        assert_eq!(
            float("1."),
            Ok((
                "",
                Float {
                    is_negative: false,
                    integer: "1",
                    fraction: None,
                    exponent_is_negative: false,
                    exponent: None,
                }
            ))
        );
        assert_eq!(
            float("1.e2"),
            Ok((
                "",
                Float {
                    is_negative: false,
                    integer: "1",
                    fraction: None,
                    exponent_is_negative: false,
                    exponent: Some("2"),
                }
            ))
        );
        assert_eq!(
            float(".7"),
            Ok((
                "",
                Float {
                    is_negative: false,
                    integer: "0",
                    fraction: Some("7"),
                    exponent_is_negative: false,
                    exponent: None,
                }
            ))
        );
        assert_eq!(
            float(".7e-4"),
            Ok((
                "",
                Float {
                    is_negative: false,
                    integer: "0",
                    fraction: Some("7"),
                    exponent_is_negative: true,
                    exponent: Some("4"),
                }
            ))
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
