use crate::{helpers::is_alphanumeric_or_underscore_or_dash, nodes::Key};
use prettify::{join, string, PrettifyDoc};
use prettify_shared::string::format_string;

fn format_keys(keys: Vec<Key>) -> PrettifyDoc {
    join(
        keys.iter()
            .map(|key| {
                if key.quoted {
                    if key.value.len() > 1
                        && key.value.chars().all(is_alphanumeric_or_underscore_or_dash)
                    {
                        string(key.value)
                    } else {
                        format_string(key.value)
                    }
                } else {
                    string(key.value)
                }
            })
            .collect(),
        string("."),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify::print;

    #[test]
    fn bare_key_test() {
        assert_eq!(
            print(format_keys(vec![Key {
                value: "foo",
                quoted: false,
            }])),
            "foo"
        );
    }

    #[test]
    fn quoted_key_test() {
        assert_eq!(
            print(format_keys(vec![Key {
                value: "foo bar",
                quoted: true,
            }])),
            "\"foo bar\""
        );
        assert_eq!(
            print(format_keys(vec![Key {
                value: "quoted \"value\"",
                quoted: true,
            }])),
            "'quoted \"value\"'"
        );
        assert_eq!(
            print(format_keys(vec![Key {
                value: "",
                quoted: true,
            }])),
            "\"\""
        );
        assert_eq!(
            print(format_keys(vec![Key {
                value: "foo_bar",
                quoted: true,
            }])),
            "foo_bar"
        );
        assert_eq!(
            print(format_keys(vec![Key {
                value: "foo-bar",
                quoted: true,
            }])),
            "foo-bar"
        );
    }

    #[test]
    fn joined_key_test() {
        assert_eq!(
            print(format_keys(vec![
                Key {
                    value: "physical",
                    quoted: false,
                },
                Key {
                    value: "color",
                    quoted: false,
                }
            ])),
            "physical.color"
        );
        assert_eq!(
            print(format_keys(vec![
                Key {
                    value: "site",
                    quoted: false,
                },
                Key {
                    value: "google.com",
                    quoted: true,
                }
            ])),
            "site.\"google.com\""
        );
    }
}
