use nom::{
    character::complete::multispace0,
    combinator::{eof, map},
    sequence::tuple,
};
use prettify::{concat, hard_line, PrettifyDoc};
use value::value;

mod array;
mod object;
mod string;
mod value;

pub fn format_json(json: &str) -> Result<PrettifyDoc, &str> {
    match map(
        tuple((multispace0, value, multispace0, eof)),
        |(_, content, _, _)| concat(vec![content, hard_line()]),
    )(json)
    {
        Ok((_, doc)) => Ok(doc),
        Err(_) => Err("Invalid json"),
    }
}
