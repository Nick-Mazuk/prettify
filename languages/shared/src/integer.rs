use crate::helpers::{opt_sign, trim_value};
use crate::{add_integer_underscores, add_integer_underscores_every_n};
use nom::combinator::{cond, map, map_res};
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag_no_case},
    character::complete::multispace1,
    combinator::{eof, peek},
    sequence::tuple,
};
use prettify::{concat, string, PrettifyDoc};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct IntegerOptions {
    use_underscores: bool,
    use_underscores_every_n: Option<usize>,
    parse_hex: bool,
    parse_octal: bool,
    parse_binary: bool,
}

impl IntegerOptions {
    pub fn new() -> Self {
        IntegerOptions {
            use_underscores: false,
            use_underscores_every_n: None,
            parse_hex: false,
            parse_octal: false,
            parse_binary: false,
        }
    }

    pub fn use_underscores(mut self) -> Self {
        self.use_underscores = true;
        self
    }

    pub fn use_underscores_every_n(mut self, n: usize) -> Self {
        self.use_underscores_every_n = Some(n);
        self.use_underscores = true;
        self
    }

    pub fn parse_hex(mut self) -> Self {
        self.parse_hex = true;
        self
    }

    pub fn parse_octal(mut self) -> Self {
        self.parse_octal = true;
        self
    }

    pub fn parse_binary(mut self) -> Self {
        self.parse_binary = true;
        self
    }
}

/**
 Parses integers. It may be more forgiving than the specs in many languages,
 but that allows for a better developer experience. Specifically, though,
 this integer parser follows the JavaScript spec for integers.

 This function parses several types of integers:

 - Decimal integers: `0`, `123`, `-123`, `+123`, `1_2__34`
 - Binary integers: `0b`, `0b0101`, `0b_0101`, `0b_0101_`, `0b_0101_0`
 - Octal integers: `0o`, `0o0101`, `0o_0101`, `0o_0101_`, `0o_0101_0`
 - Hexadecimal integers: `0x`, `0x0101`, `0x_0101`, `0x_0101_`, `0x_0101_0`
*/
pub fn integer<'a>(
    options: IntegerOptions,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    alt((
        map_res(
            cond(options.parse_binary, binary_integer(options)),
            |result| {
                result.ok_or(nom::error::Error {
                    code: nom::error::ErrorKind::MapRes,
                    input: "",
                })
            },
        ),
        map_res(
            cond(options.parse_octal, octal_integer(options)),
            |result| {
                result.ok_or(nom::error::Error {
                    code: nom::error::ErrorKind::MapRes,
                    input: "",
                })
            },
        ),
        map_res(
            cond(options.parse_hex, hexadecimal_integer(options)),
            |result| {
                result.ok_or(nom::error::Error {
                    code: nom::error::ErrorKind::MapRes,
                    input: "",
                })
            },
        ),
        decimal_integer(options),
    ))
}

fn decimal_integer<'a>(
    options: IntegerOptions,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    map(
        tuple((opt_sign, is_a("0123456789_"))),
        move |(sign, mut value)| {
            value = trim_value(value);
            concat(vec![
                sign,
                string(if options.use_underscores {
                    match options.use_underscores_every_n {
                        Some(n) => add_integer_underscores_every_n(value, n),
                        None => add_integer_underscores(value),
                    }
                } else {
                    value.replace('_', "")
                }),
            ])
        },
    )
}

fn hexadecimal_integer<'a>(
    options: IntegerOptions,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    map(
        tuple((
            opt_sign,
            tag_no_case("0x"),
            alt((
                is_a("abcdefABCDEF0123456789_"),
                peek(multispace1),
                peek(eof),
            )),
        )),
        move |(sign, _, mut value)| {
            value = trim_value(value);
            concat(vec![
                sign,
                string("0x"),
                string(
                    if options.use_underscores {
                        add_integer_underscores_every_n(trim_value(value), 4)
                    } else {
                        value.replace('_', "")
                    }
                    .to_ascii_lowercase(),
                ),
            ])
        },
    )
}

fn octal_integer<'a>(
    options: IntegerOptions,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    map(
        tuple((
            opt_sign,
            tag_no_case("0o"),
            alt((is_a("01234567_"), peek(multispace1), peek(eof))),
        )),
        move |(sign, _, mut value)| {
            value = trim_value(value);
            concat(vec![
                sign,
                string("0o"),
                string(if options.use_underscores {
                    add_integer_underscores_every_n(trim_value(value), 4)
                } else {
                    value.replace('_', "")
                }),
            ])
        },
    )
}

fn binary_integer<'a>(
    options: IntegerOptions,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    map(
        tuple((
            opt_sign,
            tag_no_case("0b"),
            alt((is_a("01_"), peek(multispace1), peek(eof))),
        )),
        move |(sign, _, mut value)| {
            value = trim_value(value);
            concat(vec![
                sign,
                string("0b"),
                string(if options.use_underscores {
                    add_integer_underscores_every_n(trim_value(value), 8)
                } else {
                    value.replace('_', "")
                }),
            ])
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_formatted;

    #[test]
    fn binary_integer_test() {
        let options = IntegerOptions::new().parse_binary().use_underscores();
        assert_formatted(integer(options)("0b1010"), ("", "0b1010"));
        assert_formatted(integer(options)("-0b1010"), ("", "-0b1010"));
        assert_formatted(integer(options)("+0b1010"), ("", "0b1010"));
        assert_formatted(integer(options)("0b"), ("", "0b0"));
        assert_formatted(integer(options)("0b "), (" ", "0b0"));
        assert_formatted(integer(options)("0b__0_"), ("", "0b0"));
        assert_formatted(integer(options)("0b_1_0_"), ("", "0b1_0"));
        assert_formatted(integer(options)("0b0001"), ("", "0b1"));
        assert_formatted(integer(options)("0b111111"), ("", "0b111111"));
        assert_formatted(
            integer(options)("0b1111111100000000"),
            ("", "0b11111111_00000000"),
        );

        assert_formatted(integer(IntegerOptions::new())("0b1111"), ("b1111", "0"));
    }

    #[test]
    fn hexadecimal_integer_test() {
        let options = IntegerOptions::new().parse_hex().use_underscores();
        assert_formatted(integer(options)("0x10"), ("", "0x10"));
        assert_formatted(integer(options)("0xDEADBEEF"), ("", "0xdead_beef"));
        assert_formatted(integer(options)("-0xdeadbeef"), ("", "-0xdead_beef"));
        assert_formatted(integer(options)("-0xdead_beef"), ("", "-0xdead_beef"));
        assert_formatted(integer(options)("-0x10"), ("", "-0x10"));
        assert_formatted(integer(options)("+0x10"), ("", "0x10"));
        assert_formatted(integer(options)("0x0001"), ("", "0x1"));
        assert_formatted(integer(options)("0x"), ("", "0x0"));
        assert_formatted(integer(options)("0x "), (" ", "0x0"));
        assert_formatted(integer(options)("0x__0_"), ("", "0x0"));
        assert_formatted(integer(options)("0x_1_0_"), ("", "0x1_0"));

        assert_formatted(integer(IntegerOptions::new())("0x1234"), ("x1234", "0"));
    }

    #[test]
    fn octal_integer_test() {
        let options = IntegerOptions::new().parse_octal().use_underscores();
        assert_formatted(integer(options)("0o10"), ("", "0o10"));
        assert_formatted(integer(options)("0o1234"), ("", "0o1234"));
        assert_formatted(integer(options)("-0o1234"), ("", "-0o1234"));
        assert_formatted(integer(options)("-0o12_34"), ("", "-0o12_34"));
        assert_formatted(integer(options)("-0o12_"), ("", "-0o12"));
        assert_formatted(integer(options)("0o0001"), ("", "0o1"));
        assert_formatted(integer(options)("-0o"), ("", "-0o0"));
        assert_formatted(integer(options)("+0o12_"), ("", "0o12"));
        assert_formatted(integer(options)("0o"), ("", "0o0"));
        assert_formatted(integer(options)("0o1234567"), ("", "0o123_4567"));

        assert_formatted(integer(IntegerOptions::new())("0o1234"), ("o1234", "0"));
    }

    #[test]
    fn decimal_integer_test() {
        let options = IntegerOptions::new().use_underscores();
        assert_formatted(integer(options)("10"), ("", "10"));
        assert_formatted(integer(options)("1234"), ("", "1_234"));
        assert_formatted(integer(options)("-1234"), ("", "-1_234"));
        assert_formatted(integer(options)("-12_34"), ("", "-12_34"));
        assert_formatted(integer(options)("-12_"), ("", "-12"));
        assert_formatted(integer(options)("+12_"), ("", "12"));
        assert_formatted(integer(options)("+12345"), ("", "12_345"));
        assert_formatted(integer(options)("0001"), ("", "1"));
    }

    #[test]
    fn do_not_use_underscores() {
        let options = IntegerOptions::new()
            .parse_binary()
            .parse_hex()
            .parse_octal();
        assert_formatted(integer(options)("10"), ("", "10"));
        assert_formatted(integer(options)("1234"), ("", "1234"));
        assert_formatted(integer(options)("-1234"), ("", "-1234"));
        assert_formatted(integer(options)("-12_34"), ("", "-1234"));
        assert_formatted(integer(options)("-12_"), ("", "-12"));
        assert_formatted(integer(options)("+12_"), ("", "12"));
        assert_formatted(integer(options)("+12345"), ("", "12345"));
        assert_formatted(integer(options)("0001"), ("", "1"));
        assert_formatted(integer(options)("0o01234567"), ("", "0o1234567"));
        assert_formatted(integer(options)("0o0123_4567"), ("", "0o1234567"));
        assert_formatted(integer(options)("0x12345678"), ("", "0x12345678"));
        assert_formatted(integer(options)("0x1234_5678"), ("", "0x12345678"));
        assert_formatted(integer(options)("0b101010_1010"), ("", "0b1010101010"));
        assert_formatted(integer(options)("0b1010101010"), ("", "0b1010101010"));
    }

    #[test]
    fn custom_underscore_every_n() {
        assert_formatted(
            integer(IntegerOptions::new().use_underscores_every_n(4))("123456789"),
            ("", "1_2345_6789"),
        );
        assert_formatted(
            integer(IntegerOptions::new().use_underscores_every_n(2))("123456789"),
            ("", "1_23_45_67_89"),
        );
    }
}
