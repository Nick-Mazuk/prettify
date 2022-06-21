use prettify::{conditional_group, print, string};

#[test]
fn conditional_group_command() {
    assert_eq!(
        print(conditional_group(
            vec![
                string("this is a very, very long line that is definitely over the eighty character limit for a single line so it should not be displayed."),
                string("this is shorter.")
            ],
            "group"
        )),
        "this is shorter.".to_string()
    );
}
