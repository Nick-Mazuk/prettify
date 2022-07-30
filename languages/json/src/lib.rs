use prettify::PrettifyDoc;
use value::value;

mod string;
mod value;

pub fn format_json(json: &str) -> Result<PrettifyDoc, &str> {
    let parsed_json = value(json);
    match parsed_json {
        Ok((_, doc)) => Ok(doc),
        Err(_) => Err("Invalid json"),
    }
}
