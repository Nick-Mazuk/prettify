use super::super::nodes::{Leaf, Leaves};
use prettify::{join, string, PrettifyDoc};

pub fn format_leaves<'a>(leaves: Leaves) -> PrettifyDoc<'a> {
    join(
        leaves.into_iter().map(|leaf| format_leaf(leaf)).collect(),
        string(" "),
    )
}

pub fn format_leaf<'a>(leaf: Leaf) -> PrettifyDoc<'a> {
    match leaf {
        Leaf::String(str) => string(str),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettify::print;

    #[test]
    fn format_strings() {
        assert_eq!(
            print(format_leaves(vec![Leaf::String("hello world".to_string())])),
            "hello world"
        );
        assert_eq!(
            print(format_leaves(vec![
                Leaf::String("hello".to_string()),
                Leaf::String("world".to_string())
            ])),
            "hello world"
        );
    }
}
