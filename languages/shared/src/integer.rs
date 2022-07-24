use crate::helpers::{opt_sign, trim_value};
use crate::{add_integer_underscores, add_integer_underscores_every_n};
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag_no_case},
    character::complete::multispace1,
    combinator::{eof, peek},
    sequence::tuple,
};
use prettify::{concat, string, PrettifyDoc};

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
pub fn integer(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    alt((
        binary_integer,
        octal_integer,
        hexadecimal_integer,
        decimal_integer,
    ))(input)
}

fn decimal_integer(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (input, (sign, value)) = tuple((opt_sign, is_a("0123456789_")))(input)?;
    Ok((
        input,
        concat(vec![
            sign,
            string(add_integer_underscores(trim_value(value))),
        ]),
    ))
}

fn hexadecimal_integer(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (input, (sign, _, value)) = tuple((
        opt_sign,
        tag_no_case("0x"),
        alt((
            is_a("abcdefABCDEF0123456789_"),
            peek(multispace1),
            peek(eof),
        )),
    ))(input)?;
    Ok((
        input,
        concat(vec![
            sign,
            string("0x"),
            string(add_integer_underscores_every_n(trim_value(value), 4).to_ascii_lowercase()),
        ]),
    ))
}

fn octal_integer(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (input, (sign, _, value)) = tuple((
        opt_sign,
        tag_no_case("0o"),
        alt((is_a("01234567_"), peek(multispace1), peek(eof))),
    ))(input)?;
    Ok((
        input,
        concat(vec![
            sign,
            string("0o"),
            string(add_integer_underscores_every_n(trim_value(value), 4)),
        ]),
    ))
}

fn binary_integer(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (input, (sign, _, value)) = tuple((
        opt_sign,
        tag_no_case("0b"),
        alt((is_a("01_"), peek(multispace1), peek(eof))),
    ))(input)?;
    Ok((
        input,
        concat(vec![
            sign,
            string("0b"),
            string(add_integer_underscores_every_n(trim_value(value), 8)),
        ]),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_formatted;

    #[test]
    fn binary_integer_test() {
        assert_formatted(integer("0b1010"), ("", "0b1010"));
        assert_formatted(integer("-0b1010"), ("", "-0b1010"));
        assert_formatted(integer("+0b1010"), ("", "0b1010"));
        assert_formatted(integer("0b"), ("", "0b0"));
        assert_formatted(integer("0b "), (" ", "0b0"));
        assert_formatted(integer("0b__0_"), ("", "0b0"));
        assert_formatted(integer("0b_1_0_"), ("", "0b1_0"));
        assert_formatted(integer("0b0001"), ("", "0b1"));
        assert_formatted(integer("0b111111"), ("", "0b111111"));
        assert_formatted(integer("0b1111111100000000"), ("", "0b11111111_00000000"));
    }

    #[test]
    fn hexadecimal_integer_test() {
        assert_formatted(integer("0x10"), ("", "0x10"));
        assert_formatted(integer("0xDEADBEEF"), ("", "0xdead_beef"));
        assert_formatted(integer("-0xdeadbeef"), ("", "-0xdead_beef"));
        assert_formatted(integer("-0xdead_beef"), ("", "-0xdead_beef"));
        assert_formatted(integer("-0x10"), ("", "-0x10"));
        assert_formatted(integer("+0x10"), ("", "0x10"));
        assert_formatted(integer("0x"), ("", "0x0"));
        assert_formatted(integer("0x "), (" ", "0x0"));
        assert_formatted(integer("0x__0_"), ("", "0x0"));
        assert_formatted(integer("0x_1_0_"), ("", "0x1_0"));
    }

    #[test]
    fn octal_integer_test() {
        assert_formatted(integer("0o10"), ("", "0o10"));
        assert_formatted(integer("0o1234"), ("", "0o1234"));
        assert_formatted(integer("-0o1234"), ("", "-0o1234"));
        assert_formatted(integer("-0o12_34"), ("", "-0o12_34"));
        assert_formatted(integer("-0o12_"), ("", "-0o12"));
        assert_formatted(integer("-0o"), ("", "-0o0"));
        assert_formatted(integer("+0o12_"), ("", "0o12"));
        assert_formatted(integer("0o"), ("", "0o0"));
        assert_formatted(integer("0o1234567"), ("", "0o123_4567"));
    }

    #[test]
    fn decimal_integer_test() {
        assert_formatted(integer("10"), ("", "10"));
        assert_formatted(integer("1234"), ("", "1_234"));
        assert_formatted(integer("-1234"), ("", "-1_234"));
        assert_formatted(integer("-12_34"), ("", "-12_34"));
        assert_formatted(integer("-12_"), ("", "-12"));
        assert_formatted(integer("+12_"), ("", "12"));
        assert_formatted(integer("+12345"), ("", "12_345"));
    }
}
