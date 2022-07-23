use crate::{helpers::is_alphanumeric_or_underscore_or_dash, nodes::Key};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::space0,
    multi::separated_list1,
    sequence::delimited,
};

pub fn key(input: &str) -> nom::IResult<&str, Vec<Key>> {
    separated_list1(
        delimited(space0, tag("."), space0),
        alt((quoted_key, bare_key)),
    )(input)
}

fn bare_key(input: &str) -> nom::IResult<&str, Key> {
    let (remainder, value) = take_while1(is_alphanumeric_or_underscore_or_dash)(input)?;
    Ok((
        remainder,
        Key {
            value,
            quoted: false,
        },
    ))
}

fn quoted_key(input: &str) -> nom::IResult<&str, Key> {
    let (remainder, value) = alt((
        delimited(tag("\""), take_until("\""), tag("\"")),
        delimited(tag("'"), take_until("'"), tag("'")),
    ))(input)?;
    Ok((
        remainder,
        Key {
            value,
            quoted: true,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bare_key_test() {
        assert_eq!(
            key("foo"),
            Ok((
                "",
                vec![Key {
                    value: "foo",
                    quoted: false,
                }]
            ))
        );
        assert_eq!(
            key("bare"),
            Ok((
                "",
                vec![Key {
                    value: "bare",
                    quoted: false,
                }]
            ))
        );
        assert_eq!(
            key("bare-key"),
            Ok((
                "",
                vec![Key {
                    value: "bare-key",
                    quoted: false,
                }]
            ))
        );
        assert_eq!(
            key("bare_key"),
            Ok((
                "",
                vec![Key {
                    value: "bare_key",
                    quoted: false,
                }]
            ))
        );
        assert_eq!(
            key("1234"),
            Ok((
                "",
                vec![Key {
                    value: "1234",
                    quoted: false,
                }]
            ))
        );
    }

    #[test]
    fn quoted_key_test() {
        assert_eq!(
            key("\"127.0.0.1\""),
            Ok((
                "",
                vec![Key {
                    value: "127.0.0.1",
                    quoted: true,
                }]
            ))
        );
        assert_eq!(
            key("'127.0.0.1'"),
            Ok((
                "",
                vec![Key {
                    value: "127.0.0.1",
                    quoted: true,
                }]
            ))
        );
        assert_eq!(
            key("\"character encoding\""),
            Ok((
                "",
                vec![Key {
                    value: "character encoding",
                    quoted: true,
                }]
            ))
        );
        assert_eq!(
            key("\"ʎǝʞ\""),
            Ok((
                "",
                vec![Key {
                    value: "ʎǝʞ",
                    quoted: true,
                }]
            ))
        );
        assert_eq!(
            key("'quoted \"value\"'"),
            Ok((
                "",
                vec![Key {
                    value: "quoted \"value\"",
                    quoted: true,
                }]
            ))
        );
        assert_eq!(
            key("\"\""),
            Ok((
                "",
                vec![Key {
                    value: "",
                    quoted: true,
                }]
            ))
        );
        assert_eq!(
            key("''"),
            Ok((
                "",
                vec![Key {
                    value: "",
                    quoted: true,
                }]
            ))
        );
    }

    #[test]
    fn joined_key_test() {
        assert_eq!(
            key("name"),
            Ok((
                "",
                vec![Key {
                    value: "name",
                    quoted: false,
                }]
            ))
        );
        assert_eq!(
            key("physical.color"),
            Ok((
                "",
                vec![
                    Key {
                        value: "physical",
                        quoted: false,
                    },
                    Key {
                        value: "color",
                        quoted: false,
                    }
                ]
            ))
        );
        assert_eq!(
            key("site.\"google.com\""),
            Ok((
                "",
                vec![
                    Key {
                        value: "site",
                        quoted: false,
                    },
                    Key {
                        value: "google.com",
                        quoted: true,
                    }
                ]
            ))
        );
        assert_eq!(
            key("site.\"google.com\".example.co"),
            Ok((
                "",
                vec![
                    Key {
                        value: "site",
                        quoted: false,
                    },
                    Key {
                        value: "google.com",
                        quoted: true,
                    },
                    Key {
                        value: "example",
                        quoted: false,
                    },
                    Key {
                        value: "co",
                        quoted: false,
                    }
                ]
            ))
        );
        assert_eq!(
            key("fruit. color"),
            Ok((
                "",
                vec![
                    Key {
                        value: "fruit",
                        quoted: false,
                    },
                    Key {
                        value: "color",
                        quoted: false,
                    }
                ]
            ))
        );
        assert_eq!(
            key("fruit . flavor"),
            Ok((
                "",
                vec![
                    Key {
                        value: "fruit",
                        quoted: false,
                    },
                    Key {
                        value: "flavor",
                        quoted: false,
                    }
                ]
            ))
        );
        assert_eq!(
            key("fruit     .      flavor"),
            Ok((
                "",
                vec![
                    Key {
                        value: "fruit",
                        quoted: false,
                    },
                    Key {
                        value: "flavor",
                        quoted: false,
                    }
                ]
            ))
        );
        assert_eq!(
            key("fruit\t . \t flavor"),
            Ok((
                "",
                vec![
                    Key {
                        value: "fruit",
                        quoted: false,
                    },
                    Key {
                        value: "flavor",
                        quoted: false,
                    }
                ]
            ))
        );
        assert_eq!(
            key("3.14159"),
            Ok((
                "",
                vec![
                    Key {
                        value: "3",
                        quoted: false,
                    },
                    Key {
                        value: "14159",
                        quoted: false,
                    }
                ]
            ))
        );
    }
}
