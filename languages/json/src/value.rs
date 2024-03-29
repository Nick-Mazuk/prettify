use nom::{branch::alt, bytes::complete::tag, combinator::map};
use prettify::{string, PrettifyDoc};
use prettify_shared::{float, integer, FloatOptions, IntegerOptions};

use crate::{array::array, object::object, string::json_string};

pub fn value(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    alt((
        float(FloatOptions::new()),
        integer(IntegerOptions::new()),
        json_string,
        array,
        object,
        literals,
    ))(input)
}

fn literals(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    map(alt((tag("true"), tag("false"), tag("null"))), string)(input)
}
