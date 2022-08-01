use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, tuple},
};
use prettify::{
    break_parent, concat, group, if_break, indent, join, line, soft_line, string, PrettifyDoc,
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RepeatedItemsOptions<'a, F: FnMut(&'a str) -> nom::IResult<&'a str, PrettifyDoc<'a>>> {
    open_delimiter: &'a str,
    item_parser: F,
    separator: &'a str,
    close_delimiter: &'a str,
    allow_trailing_separator: bool,
    use_user_preferred_indentation: bool,
    use_space_around_delimiters: bool,
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
            tuple((
                multispace0,
                separated_list0(
                    tuple((multispace0, tag(options.separator), multispace0)),
                    options.item_parser,
                ),
            )),
            tuple((
                multispace0,
                opt(tag(options.separator)),
                multispace0,
                tag(options.close_delimiter),
            )),
        ),
        move |result| {
            let (initial_whitespace, items) = result;
            group(concat(vec![
                string(options.open_delimiter),
                indent(concat(vec![
                    if options.use_user_preferred_indentation && initial_whitespace.contains("\n") {
                        break_parent()
                    } else {
                        string("")
                    },
                    if options.use_space_around_delimiters {
                        line()
                    } else {
                        soft_line()
                    },
                    join(items, concat(vec![string(options.separator), line()])),
                    if options.allow_trailing_separator {
                        if_break(string(options.separator), string(""), "separator")
                    } else {
                        string("")
                    },
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
            ))("{ hello , hello }"),
            ("", "{hello, hello}"),
        );
        assert_formatted(
            repeated_items(RepeatedItemsOptions::new(
                "{",
                map(tag("hello"), string),
                ",",
                "}",
            ))("{ \n hello \n , \n hello \n , \n }"),
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

    #[test]
    fn allow_trailing_separator() {
        assert_formatted(
            repeated_items(
                RepeatedItemsOptions::new("{", map(tag("hello"), string), ",", "}")
                    .allow_trailing_separator(),
            )("{hello,hello}"),
            ("", "{hello, hello}"),
        );
        assert_formatted(
            repeated_items(RepeatedItemsOptions::new(
                "{",
                map(tag("hello"), string),
                ",",
                "}",
            ).allow_trailing_separator())("{hello,hello,hello,hello,hello,hello,hello,hello,hello,hello,hello,hello}"),
            ("", "{\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n}"),
        );
    }

    #[test]
    fn use_user_preferred_indentation() {
        assert_formatted(
            repeated_items(
                RepeatedItemsOptions::new("{", map(tag("hello"), string), ",", "}")
                    .use_user_preferred_indentation(),
            )("{hello,hello}"),
            ("", "{hello, hello}"),
        );
        assert_formatted(
            repeated_items(
                RepeatedItemsOptions::new("{", map(tag("hello"), string), ",", "}")
                    .use_user_preferred_indentation(),
            )("{\nhello,hello}"),
            ("", "{\n    hello,\n    hello\n}"),
        );
        assert_formatted(
            repeated_items(RepeatedItemsOptions::new(
                "{",
                map(tag("hello"), string),
                ",",
                "}",
            ).use_user_preferred_indentation())("{hello,hello,hello,hello,hello,hello,hello,hello,hello,hello,hello,hello}"),
            ("", "{\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello,\n    hello\n}"),
        );
    }
}
