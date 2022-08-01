use prettify::PrettifyDoc;
use value::value;

mod array;
mod object;
mod string;
mod value;

pub fn format_json(json: &str) -> Result<PrettifyDoc, &str> {
    match value(json) {
        Ok((_, doc)) => Ok(doc),
        Err(_) => Err("Invalid json"),
    }
}
