use prettify::{concat, string, PrettifyDoc};

use crate::{RFC3339Date, RFC3339DateTime, RFC3339PartialTime, RFC3339Time, RFC3339TimeOffset};

pub fn format_rfc_3339_full_year(year: &str) -> PrettifyDoc {
    string(year)
}

pub fn format_rfc_3339_month(month: &str) -> PrettifyDoc {
    string(month)
}

pub fn format_rfc_3339_day(day: &str) -> PrettifyDoc {
    string(day)
}

pub fn format_rfc_3339_hour(hour: &str) -> PrettifyDoc {
    string(hour)
}

pub fn format_rfc_3339_minute(minute: &str) -> PrettifyDoc {
    string(minute)
}

pub fn format_rfc_3339_second(second: &str) -> PrettifyDoc {
    string(second)
}

pub fn format_rfc_3339_second_subfraction(subfraction: &str) -> PrettifyDoc {
    concat(vec![string("."), string(subfraction)])
}

pub fn format_rfc_3339_time_offset(timezone: RFC3339TimeOffset) -> PrettifyDoc {
    match timezone {
        RFC3339TimeOffset::Z => string("Z"),
        RFC3339TimeOffset::NumberOffset(hour, minute, is_negative) => concat(vec![
            string(if is_negative { "-" } else { "+" }),
            string(hour),
            string(":"),
            string(minute),
        ]),
    }
}

pub fn format_rfc_3339_partial_time(time: RFC3339PartialTime) -> PrettifyDoc {
    let mut doc = vec![
        string(time.hour),
        string(":"),
        string(time.minute),
        string(":"),
        string(time.second),
    ];
    if let Some(subfraction) = &time.subfraction {
        doc.push(format_rfc_3339_second_subfraction(subfraction));
    }
    concat(doc)
}

pub fn format_rfc_3339_time(time: RFC3339Time) -> PrettifyDoc {
    concat(vec![
        format_rfc_3339_partial_time(time.partial),
        format_rfc_3339_time_offset(time.offset),
    ])
}

pub fn format_rfc_3339_date(date: RFC3339Date) -> PrettifyDoc {
    concat(vec![
        format_rfc_3339_full_year(date.year),
        string("-"),
        format_rfc_3339_month(date.month),
        string("-"),
        format_rfc_3339_day(date.day),
    ])
}

pub fn format_rfc_3339_date_time(date_time: RFC3339DateTime) -> PrettifyDoc {
    concat(vec![
        format_rfc_3339_date(date_time.date),
        string("T"),
        format_rfc_3339_time(date_time.time),
    ])
}

#[cfg(test)]
mod test {
    use super::*;
    use prettify::print;

    #[test]
    fn test_format_rfc_3339_full_year() {
        assert_eq!(print(format_rfc_3339_full_year("2020")), "2020");
    }

    #[test]
    fn test_format_rfc_3339_month() {
        assert_eq!(print(format_rfc_3339_month("01")), "01");
    }

    #[test]
    fn test_format_rfc_3339_day() {
        assert_eq!(print(format_rfc_3339_day("01")), "01");
    }

    #[test]
    fn test_format_rfc_3339_hour() {
        assert_eq!(print(format_rfc_3339_hour("01")), "01");
    }

    #[test]
    fn test_format_rfc_3339_minute() {
        assert_eq!(print(format_rfc_3339_minute("01")), "01");
    }

    #[test]
    fn test_format_rfc_3339_second() {
        assert_eq!(print(format_rfc_3339_second("01")), "01");
    }

    #[test]
    fn test_format_rfc_3339_second_subfraction() {
        assert_eq!(print(format_rfc_3339_second_subfraction("123")), ".123");
    }

    #[test]
    fn test_format_rfc_3339_time_offset() {
        assert_eq!(
            print(format_rfc_3339_time_offset(RFC3339TimeOffset::Z)),
            "Z"
        );
        assert_eq!(
            print(format_rfc_3339_time_offset(
                RFC3339TimeOffset::NumberOffset("01", "02", false)
            )),
            "+01:02"
        );
        assert_eq!(
            print(format_rfc_3339_time_offset(
                RFC3339TimeOffset::NumberOffset("01", "02", true)
            )),
            "-01:02"
        );
    }

    #[test]
    fn test_format_rfc_3339_partial_time() {
        assert_eq!(
            print(format_rfc_3339_partial_time(RFC3339PartialTime {
                hour: "01",
                minute: "02",
                second: "03",
                subfraction: Some("123"),
            })),
            "01:02:03.123"
        );
        assert_eq!(
            print(format_rfc_3339_partial_time(RFC3339PartialTime {
                hour: "01",
                minute: "02",
                second: "03",
                subfraction: None,
            })),
            "01:02:03"
        );
    }

    #[test]
    fn test_format_rfc_3339_time() {
        assert_eq!(
            print(format_rfc_3339_time(RFC3339Time {
                partial: RFC3339PartialTime {
                    hour: "01",
                    minute: "02",
                    second: "03",
                    subfraction: Some("123"),
                },
                offset: RFC3339TimeOffset::Z,
            })),
            "01:02:03.123Z"
        );
        assert_eq!(
            print(format_rfc_3339_time(RFC3339Time {
                partial: RFC3339PartialTime {
                    hour: "01",
                    minute: "02",
                    second: "03",
                    subfraction: None,
                },
                offset: RFC3339TimeOffset::Z,
            })),
            "01:02:03Z"
        );
        assert_eq!(
            print(format_rfc_3339_time(RFC3339Time {
                partial: RFC3339PartialTime {
                    hour: "01",
                    minute: "02",
                    second: "03",
                    subfraction: Some("123"),
                },
                offset: RFC3339TimeOffset::NumberOffset("01", "02", false),
            })),
            "01:02:03.123+01:02"
        );
        assert_eq!(
            print(format_rfc_3339_time(RFC3339Time {
                partial: RFC3339PartialTime {
                    hour: "01",
                    minute: "02",
                    second: "03",
                    subfraction: Some("123"),
                },
                offset: RFC3339TimeOffset::NumberOffset("01", "02", true),
            })),
            "01:02:03.123-01:02"
        );
    }

    #[test]
    fn test_format_rfc_3339_date() {
        assert_eq!(
            print(format_rfc_3339_date(RFC3339Date {
                year: "2020",
                month: "01",
                day: "02",
            })),
            "2020-01-02"
        );
    }

    #[test]
    fn test_format_rfc_3339_date_time() {
        assert_eq!(
            print(format_rfc_3339_date_time(RFC3339DateTime {
                date: RFC3339Date {
                    year: "2020",
                    month: "01",
                    day: "02",
                },
                time: RFC3339Time {
                    partial: RFC3339PartialTime {
                        hour: "01",
                        minute: "02",
                        second: "03",
                        subfraction: Some("123"),
                    },
                    offset: RFC3339TimeOffset::Z,
                },
            })),
            "2020-01-02T01:02:03.123Z"
        );
        assert_eq!(
            print(format_rfc_3339_date_time(RFC3339DateTime {
                date: RFC3339Date {
                    year: "2020",
                    month: "01",
                    day: "02",
                },
                time: RFC3339Time {
                    partial: RFC3339PartialTime {
                        hour: "01",
                        minute: "02",
                        second: "03",
                        subfraction: None,
                    },
                    offset: RFC3339TimeOffset::NumberOffset("01", "02", false),
                },
            })),
            "2020-01-02T01:02:03+01:02"
        );
    }
}
