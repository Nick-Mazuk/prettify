use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, tuple},
};
use prettify::{concat, group, indent, join, line, soft_line, string, PrettifyDoc};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RepeatedItemsOptions<'a, F: FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>>> {
    pub open_delimiter: &'a str,
    pub item_parser: F,
    pub separator: &'a str,
    pub close_delimiter: &'a str,
    pub allow_trailing_separator: bool,
    pub use_user_preferred_indentation: bool,
    pub use_space_around_delimiters: bool,
}

impl<'a, F: FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>>> RepeatedItemsOptions<'a, F> {
    pub fn new(
        open_delimiter: &'a str,
        item_parser: F,
        separator: &'a str,
        close_delimiter: &'a str,
    ) -> RepeatedItemsOptions<'a, F> {
        RepeatedItemsOptions {
            open_delimiter,
            item_parser,
            separator,
            close_delimiter,
            allow_trailing_separator: false,
            use_user_preferred_indentation: false,
            use_space_around_delimiters: false,
        }
    }

    pub fn allow_trailing_separator(mut self) -> Self {
        self.allow_trailing_separator = true;
        self
    }

    pub fn use_user_preferred_indentation(mut self) -> Self {
        self.use_user_preferred_indentation = true;
        self
    }

    pub fn use_space_around_delimiters(mut self) -> Self {
        self.use_space_around_delimiters = true;
        self
    }
}

pub fn repeated_items<'a, F: FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>>>(
    options: RepeatedItemsOptions<'a, F>,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>> {
    map(
        delimited(
            tag(options.open_delimiter),
            separated_list0(
                tuple((multispace0, tag(options.separator), multispace0)),
                options.item_parser,
            ),
            tag(options.close_delimiter),
        ),
        move |result| {
            group(concat(vec![
                string(options.open_delimiter),
                indent(concat(vec![
                    if options.use_space_around_delimiters {
                        line()
                    } else {
                        soft_line()
                    },
                    join(result, concat(vec![string(options.separator), line()])),
                ])),
                if options.use_space_around_delimiters {
                    line()
                } else {
                    soft_line()
                },
                string(options.close_delimiter),
            ]))
        },
    )
}

#[cfg(test)]
mod test {
    use crate::assert_formatted;

    use super::*;

    #[test]
    fn test_default_behavior() {
        assert_formatted(
            repeated_items(RepeatedItemsOptions::new(
                "{",
                map(tag("hello"), string),
                ",",
                "}",
            ))("{hello,hello}"),
            ("", "{hello, hello}"),
        );
        assert_formatted(
            repeated_items(RepeatedItemsOptions::new(
                "{",
                map(tag("hello"), string),
                ",",
                "}",
            ))("{hello,hello,hello,hello,hello,hello,hello,hello,hello,hello,hello,hello}"),
            ("", "{\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello\n}"),
        );
    }

    #[test]
    fn space_around_delimiters() {
        assert_formatted(
            repeated_items(
                RepeatedItemsOptions::new("{", map(tag("hello"), string), ",", "}")
                    .use_space_around_delimiters(),
            )("{hello,hello}"),
            ("", "{ hello, hello }"),
        );
        assert_formatted(
            repeated_items(RepeatedItemsOptions::new(
                "{",
                map(tag("hello"), string),
                ",",
                "}",
            ).use_space_around_delimiters())("{hello,hello,hello,hello,hello,hello,hello,hello,hello,hello,hello,hello}"),
            ("", "{\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello\n}"),
        );
    }
}
