use crate::nodes::{Integer, IntegerKind};
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, tag_no_case},
    character::complete::multispace1,
    combinator::{eof, opt, peek},
    sequence::tuple,
};

pub fn integer(input: &str) -> nom::IResult<&str, Integer> {
    alt((
        binary_integer,
        octal_integer,
        hexadecimal_integer,
        decimal_integer,
    ))(input)
}

fn trim_value(input: &str) -> &str {
    let value = input
        .trim_end_matches('_')
        .trim_start_matches(|char| char == '_' || char == '0' || char == ' ');
    if value == "" {
        "0"
    } else {
        value
    }
}

fn decimal_integer(input: &str) -> nom::IResult<&str, Integer> {
    let (input, (sign, value)) =
        tuple((opt(alt((tag("-"), tag("+")))), is_a("0123456789_")))(input)?;
    let is_negative = match sign {
        Some("-") => true,
        _ => false,
    };
    Ok((
        input,
        Integer {
            is_negative,
            value: trim_value(value),
            kind: IntegerKind::Decimal,
        },
    ))
}

fn hexadecimal_integer(input: &str) -> nom::IResult<&str, Integer> {
    let (input, (sign, _, value)) = tuple((
        opt(alt((tag("-"), tag("+")))),
        tag_no_case("0x"),
        alt((
            is_a("abcdefABCDEF0123456789_"),
            peek(multispace1),
            peek(eof),
        )),
    ))(input)?;
    let is_negative = match sign {
        Some("-") => true,
        _ => false,
    };
    Ok((
        input,
        Integer {
            is_negative,
            value: trim_value(value),
            kind: IntegerKind::Hexadecimal,
        },
    ))
}

fn octal_integer(input: &str) -> nom::IResult<&str, Integer> {
    let (input, (sign, _, value)) = tuple((
        opt(alt((tag("-"), tag("+")))),
        tag_no_case("0o"),
        alt((is_a("01234567_"), peek(multispace1), peek(eof))),
    ))(input)?;
    let is_negative = match sign {
        Some("-") => true,
        _ => false,
    };
    Ok((
        input,
        Integer {
            is_negative,
            value: trim_value(value),
            kind: IntegerKind::Octal,
        },
    ))
}

fn binary_integer(input: &str) -> nom::IResult<&str, Integer> {
    let (input, (sign, _, value)) = tuple((
        opt(alt((tag("-"), tag("+")))),
        tag_no_case("0b"),
        alt((is_a("01_"), peek(multispace1), peek(eof))),
    ))(input)?;
    let is_negative = match sign {
        Some("-") => true,
        _ => false,
    };
    Ok((
        input,
        Integer {
            is_negative,
            value: trim_value(value),
            kind: IntegerKind::Binary,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binary_integer_test() {
        assert_eq!(
            integer("0b1010"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "1010",
                    kind: IntegerKind::Binary,
                }
            ))
        );
        assert_eq!(
            integer("-0b1010"),
            Ok((
                "",
                Integer {
                    is_negative: true,
                    value: "1010",
                    kind: IntegerKind::Binary,
                }
            ))
        );
        assert_eq!(
            integer("+0b1010"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "1010",
                    kind: IntegerKind::Binary,
                }
            ))
        );
        assert_eq!(
            integer("0b"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "0",
                    kind: IntegerKind::Binary,
                }
            ))
        );
        assert_eq!(
            integer("0b "),
            Ok((
                " ",
                Integer {
                    is_negative: false,
                    value: "0",
                    kind: IntegerKind::Binary,
                }
            ))
        );
        assert_eq!(
            integer("0b__0_"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "0",
                    kind: IntegerKind::Binary,
                }
            ))
        );
        assert_eq!(
            integer("0b_1_0_"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "1_0",
                    kind: IntegerKind::Binary,
                }
            ))
        );
    }

    #[test]
    fn hexadecimal_integer_test() {
        assert_eq!(
            integer("0x10"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "10",
                    kind: IntegerKind::Hexadecimal,
                }
            ))
        );
        assert_eq!(
            integer("0xDEADBEEF"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "DEADBEEF",
                    kind: IntegerKind::Hexadecimal,
                }
            ))
        );
        assert_eq!(
            integer("-0xdeadbeef"),
            Ok((
                "",
                Integer {
                    is_negative: true,
                    value: "deadbeef",
                    kind: IntegerKind::Hexadecimal,
                }
            ))
        );
        assert_eq!(
            integer("-0xdead_beef"),
            Ok((
                "",
                Integer {
                    is_negative: true,
                    value: "dead_beef",
                    kind: IntegerKind::Hexadecimal,
                }
            ))
        );
        assert_eq!(
            integer("-0x10"),
            Ok((
                "",
                Integer {
                    is_negative: true,
                    value: "10",
                    kind: IntegerKind::Hexadecimal,
                }
            ))
        );
        assert_eq!(
            integer("+0x10"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "10",
                    kind: IntegerKind::Hexadecimal,
                }
            ))
        );
        assert_eq!(
            integer("0x"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "0",
                    kind: IntegerKind::Hexadecimal,
                }
            ))
        );
        assert_eq!(
            integer("0x "),
            Ok((
                " ",
                Integer {
                    is_negative: false,
                    value: "0",
                    kind: IntegerKind::Hexadecimal,
                }
            ))
        );
        assert_eq!(
            integer("0x__0_"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "0",
                    kind: IntegerKind::Hexadecimal,
                }
            ))
        );
        assert_eq!(
            integer("0x_1_0_"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "1_0",
                    kind: IntegerKind::Hexadecimal,
                }
            ))
        );
    }

    #[test]
    fn octal_integer_test() {
        assert_eq!(
            integer("0o10"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "10",
                    kind: IntegerKind::Octal,
                }
            ))
        );
        assert_eq!(
            integer("0o1234"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "1234",
                    kind: IntegerKind::Octal,
                }
            ))
        );
        assert_eq!(
            integer("-0o1234"),
            Ok((
                "",
                Integer {
                    is_negative: true,
                    value: "1234",
                    kind: IntegerKind::Octal,
                }
            ))
        );
        assert_eq!(
            integer("-0o12_34"),
            Ok((
                "",
                Integer {
                    is_negative: true,
                    value: "12_34",
                    kind: IntegerKind::Octal,
                }
            ))
        );
        assert_eq!(
            integer("-0o12_"),
            Ok((
                "",
                Integer {
                    is_negative: true,
                    value: "12",
                    kind: IntegerKind::Octal,
                }
            ))
        );
        assert_eq!(
            integer("-0o"),
            Ok((
                "",
                Integer {
                    is_negative: true,
                    value: "0",
                    kind: IntegerKind::Octal,
                }
            ))
        );
        assert_eq!(
            integer("+0o12_"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "12",
                    kind: IntegerKind::Octal,
                }
            ))
        );
        assert_eq!(
            integer("0o"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "0",
                    kind: IntegerKind::Octal,
                }
            ))
        );
    }

    #[test]
    fn decimal_integer_test() {
        assert_eq!(
            integer("10"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "10",
                    kind: IntegerKind::Decimal,
                }
            ))
        );
        assert_eq!(
            integer("1234"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "1234",
                    kind: IntegerKind::Decimal,
                }
            ))
        );
        assert_eq!(
            integer("-1234"),
            Ok((
                "",
                Integer {
                    is_negative: true,
                    value: "1234",
                    kind: IntegerKind::Decimal,
                }
            ))
        );
        assert_eq!(
            integer("-12_34"),
            Ok((
                "",
                Integer {
                    is_negative: true,
                    value: "12_34",
                    kind: IntegerKind::Decimal,
                }
            ))
        );
        assert_eq!(
            integer("-12_"),
            Ok((
                "",
                Integer {
                    is_negative: true,
                    value: "12",
                    kind: IntegerKind::Decimal,
                }
            ))
        );
        assert_eq!(
            integer("+12_"),
            Ok((
                "",
                Integer {
                    is_negative: false,
                    value: "12",
                    kind: IntegerKind::Decimal,
                }
            ))
        );
    }
}
