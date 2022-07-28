use prettify::string;

extern crate nom;
extern crate prettify;

mod boolean;
mod comment;
mod helpers;
mod key;

mod string;

pub fn format_toml(_toml: &str) -> Result<prettify::PrettifyDoc, &str> {
    Ok(string("hello"))
}
