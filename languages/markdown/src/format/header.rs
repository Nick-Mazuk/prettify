use super::super::nodes::Leaves;
use super::leaf::format_leaves;
use prettify::{concat, hard_line, string, PrettifyDoc};

pub fn format_header<'a>(size: usize, leaves: Leaves) -> PrettifyDoc<'a> {
    let header_marker = "#";
    let header_marker = header_marker.repeat(size) + " ";

    concat(vec![
        string(header_marker),
        format_leaves(leaves),
        hard_line(),
    ])
}

#[cfg(test)]
mod tests {
    use super::super::super::nodes::Leaf;
    use super::*;
    use prettify::print;

    #[test]
    fn header_1() {
        assert_eq!(
            print(format_header(
                1,
                vec![Leaf::String("hello world".to_string())]
            )),
            "# hello world\n"
        );
    }
}
