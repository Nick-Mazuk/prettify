use prettify::string;

extern crate nom;
extern crate prettify;

mod boolean;
mod helpers;
mod key;
mod key_value_pair;
mod line_endings;
mod string;

pub fn format_toml(_toml: &str) -> Result<prettify::PrettifyDoc, &str> {
    Ok(string("hello"))
}
