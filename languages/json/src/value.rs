use nom::branch::alt;
use prettify::PrettifyDoc;
use prettify_shared::{float, integer};

use crate::string::json_string;

pub fn value(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    alt((
        float,
        integer,
        json_string,
        // array,
        // object,
    ))(input)
}
