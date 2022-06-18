use super::super::doc::{DocCommand, PrettifyConfig};

mod group;
mod hardline;
mod join;

pub fn print_command(command: &DocCommand, output: &mut String, options: &PrettifyConfig) {
    match command {
        DocCommand::Group(doc) => group::print_group(doc, output, options),
        DocCommand::HardLine => hardline::print_hardline(output),
        DocCommand::Join(separator, children) => {
            join::print_join(separator, children, output, options)
        }
    }
}
