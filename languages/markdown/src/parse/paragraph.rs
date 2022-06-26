use super::leaf::leaves;
use crate::nodes::Block;

pub fn paragraph(input: &str) -> nom::IResult<&str, Block> {
    let result = leaves(input);
    match result {
        Ok((remainder, leaves)) => Ok((remainder, Block::Paragraph(leaves))),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::Leaf;

    #[test]
    fn header_test() {
        assert_eq!(
            paragraph("hello world"),
            Ok((
                "",
                Block::Paragraph(vec![Leaf::String("hello world".to_string())])
            ))
        );
    }
}
