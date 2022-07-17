use prettify::{
    concat, conditional_group, fill, hard_line, join_to_vector, line, print, string, PrettifyDoc,
};

#[test]
fn conditional_group_command() {
    assert_eq!(
        print(conditional_group(
            vec![
                string("this is a very, very long line that is definitely over the eighty character limit for a single line so it should not be displayed."),
                string("this is a very, very long line that is definitely over the eighty character limit for a single line so it should not be displayed."),
                string("this is short enough so it will print."),
                string("this will not print.")
            ],
            "group"
        )),
        "this is short enough so it will print.".to_string()
    );
}

#[test]
fn prints_first_if_it_fits() {
    assert_eq!(
        print(conditional_group(
            vec![
                string("this is short and will print."),
                string("this is a very, very long line that is definitely over the eighty character limit for a single line so it should not be displayed."),
                string("this is a very, very long line that is definitely over the eighty character limit for a single line so it should not be displayed."),
                string("this is short enough so it will print."),
                string("this will not print.")
            ],
            "group"
        )),
        "this is short and will print.".to_string()
    );
}

#[test]
fn prints_only_item_if_fits() {
    assert_eq!(
        print(conditional_group(
            vec![string("this is short and will print.")],
            "group"
        )),
        "this is short and will print.".to_string()
    );
}

#[test]
fn prints_only_item_even_if_not_fit() {
    assert_eq!(
        print(conditional_group(
            vec![string("this is a very, very long line that is definitely over the eighty character limit for a single line but it's the only item so it will print."),],
            "group"
        )),
        "this is a very, very long line that is definitely over the eighty character limit for a single line but it's the only item so it will print.".to_string()
    );
}

#[test]
fn conditional_group_with_complex_blocks() {
    assert_eq!(
        print(concat(vec![
            conditional_group(
                vec![
                    concat(vec![string("## "), concat(vec![string("Heading")])]),
                    concat(vec![
                        concat(vec![string("Heading")]),
                        hard_line(),
                        string("------------")
                    ]),
                ],
                "block",
            ),
            hard_line(),
        ])),
        "## Heading\n".to_string()
    );
}
#[test]
fn conditional_group_with_complex_blocks_2() {
    let text = "this is a very, very long line that is definitely over the eighty character limit for a single line but it's the only item so it will print.";
    let words: Vec<PrettifyDoc> = text.replace('\n', " ").split(' ').map(string).collect();
    assert_eq!(
        print(concat(vec![conditional_group(
            vec![
                concat(vec![
                    string("## "),
                    concat(join_to_vector(words.clone(), line()))
                ]),
                concat(vec![
                    fill(join_to_vector(words, line())),
                    hard_line(),
                    string("------------")
                ]),
            ],
            "block",
        ),
        hard_line(),
    ])),
        "this is a very, very long line that is definitely over the eighty character limit\nfor a single line but it's the only item so it will print.\n------------\n".to_string()
    );
}
