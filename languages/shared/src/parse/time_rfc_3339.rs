use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while_m_n},
    character::complete::digit0,
    combinator::opt,
    sequence::{preceded, tuple},
};

use crate::{RFC3339Date, RFC3339DateTime, RFC3339PartialTime, RFC3339Time, RFC3339TimeOffset};

use super::helpers::{sign, sign_is_positive};

pub fn is_digit(chr: char) -> bool {
    chr >= '0' && chr <= '9'
}

pub fn rfc_3339_full_year(input: &str) -> nom::IResult<&str, &str> {
    take_while_m_n(4, 4, is_digit)(input)
}

pub fn rfc_3339_month(input: &str) -> nom::IResult<&str, &str> {
    take_while_m_n(2, 2, is_digit)(input)
}

pub fn rfc_3339_day(input: &str) -> nom::IResult<&str, &str> {
    take_while_m_n(2, 2, is_digit)(input)
}

pub fn rfc_3339_hour(input: &str) -> nom::IResult<&str, &str> {
    take_while_m_n(2, 2, is_digit)(input)
}

pub fn rfc_3339_minute(input: &str) -> nom::IResult<&str, &str> {
    take_while_m_n(2, 2, is_digit)(input)
}

pub fn rfc_3339_second(input: &str) -> nom::IResult<&str, &str> {
    take_while_m_n(2, 2, is_digit)(input)
}

pub fn rfc_3339_second_subfraction(input: &str) -> nom::IResult<&str, &str> {
    let (remainder, result) = preceded(tag("."), digit0)(input)?;
    Ok((remainder, result.trim_end_matches("0")))
}

fn time_offset_z(input: &str) -> nom::IResult<&str, RFC3339TimeOffset> {
    let (remainder, _) = tag_no_case("z")(input)?;
    Ok((remainder, RFC3339TimeOffset::Z))
}

fn time_offset_number_offset(input: &str) -> nom::IResult<&str, RFC3339TimeOffset> {
    let (remainder, (sign, hour, _, minute)) =
        tuple((sign, rfc_3339_hour, tag(":"), rfc_3339_minute))(input)?;
    Ok((
        remainder,
        RFC3339TimeOffset::NumberOffset(hour, minute, sign_is_positive(sign)),
    ))
}

pub fn rfc_3339_time_offset(input: &str) -> nom::IResult<&str, RFC3339TimeOffset> {
    alt((time_offset_z, time_offset_number_offset))(input)
}

pub fn rfc_3339_partial_time(input: &str) -> nom::IResult<&str, RFC3339PartialTime> {
    let (remainder, (hour, _, minute, _, second, subfraction)) = tuple((
        rfc_3339_hour,
        tag(":"),
        rfc_3339_minute,
        tag(":"),
        rfc_3339_second,
        opt(rfc_3339_second_subfraction),
    ))(input)?;
    Ok((
        remainder,
        RFC3339PartialTime {
            hour,
            minute,
            second,
            subfraction,
        },
    ))
}

pub fn rfc_3339_time(input: &str) -> nom::IResult<&str, RFC3339Time> {
    let (remainder, (partial, offset)) =
        tuple((rfc_3339_partial_time, rfc_3339_time_offset))(input)?;
    Ok((remainder, RFC3339Time { partial, offset }))
}

pub fn rfc_3339_date(input: &str) -> nom::IResult<&str, RFC3339Date> {
    let (remainder, (year, _, month, _, day)) = tuple((
        rfc_3339_full_year,
        tag("-"),
        rfc_3339_month,
        tag("-"),
        rfc_3339_day,
    ))(input)?;
    Ok((remainder, RFC3339Date { year, month, day }))
}

pub fn rfc_3339_date_time(input: &str) -> nom::IResult<&str, RFC3339DateTime> {
    let (remainder, (date, _, time)) = tuple((
        rfc_3339_date,
        alt((tag_no_case("T"), tag(" "))),
        rfc_3339_time,
    ))(input)?;
    Ok((remainder, RFC3339DateTime { date, time }))
}

#[cfg(test)]
mod test {
    use crate::assert_errors;

    use super::*;

    #[test]
    fn test_rfc_3339_full_year() {
        assert_eq!(rfc_3339_full_year("2020"), Ok(("", "2020")));
        assert_eq!(rfc_3339_full_year("2020-"), Ok(("-", "2020")));
        assert_eq!(rfc_3339_full_year("2020-01"), Ok(("-01", "2020")));
        assert_eq!(rfc_3339_full_year("20200"), Ok(("0", "2020")));

        assert_errors(rfc_3339_full_year("203"));
        assert_errors(rfc_3339_full_year("-203"));
    }

    #[test]
    fn test_rfc_3339_month() {
        assert_eq!(rfc_3339_month("01"), Ok(("", "01")));
        assert_eq!(rfc_3339_month("01-"), Ok(("-", "01")));
        assert_eq!(rfc_3339_month("01-01"), Ok(("-01", "01")));
        assert_eq!(rfc_3339_month("0101"), Ok(("01", "01")));

        assert_errors(rfc_3339_month("0"));
        assert_errors(rfc_3339_month("-01"));
    }

    #[test]
    fn test_rfc_3339_day() {
        assert_eq!(rfc_3339_day("01"), Ok(("", "01")));
        assert_eq!(rfc_3339_day("01-"), Ok(("-", "01")));
        assert_eq!(rfc_3339_day("01-01"), Ok(("-01", "01")));
        assert_eq!(rfc_3339_day("0101"), Ok(("01", "01")));

        assert_errors(rfc_3339_day("0"));
        assert_errors(rfc_3339_day("-01"));
    }

    #[test]
    fn test_rfc_3339_hour() {
        assert_eq!(rfc_3339_hour("01"), Ok(("", "01")));
        assert_eq!(rfc_3339_hour("01-"), Ok(("-", "01")));
        assert_eq!(rfc_3339_hour("01-01"), Ok(("-01", "01")));
        assert_eq!(rfc_3339_hour("0101"), Ok(("01", "01")));

        assert_errors(rfc_3339_hour("0"));
        assert_errors(rfc_3339_hour("-01"));
    }

    #[test]
    fn test_rfc_3339_minute() {
        assert_eq!(rfc_3339_minute("01"), Ok(("", "01")));
        assert_eq!(rfc_3339_minute("01-"), Ok(("-", "01")));
        assert_eq!(rfc_3339_minute("01-01"), Ok(("-01", "01")));
        assert_eq!(rfc_3339_minute("0101"), Ok(("01", "01")));

        assert_errors(rfc_3339_minute("0"));
        assert_errors(rfc_3339_minute("-01"));
    }

    #[test]
    fn test_rfc_3339_second() {
        assert_eq!(rfc_3339_second("01"), Ok(("", "01")));
        assert_eq!(rfc_3339_second("01-"), Ok(("-", "01")));
        assert_eq!(rfc_3339_second("01-01"), Ok(("-01", "01")));
        assert_eq!(rfc_3339_second("0101"), Ok(("01", "01")));

        assert_errors(rfc_3339_second("0"));
        assert_errors(rfc_3339_second("-01"));
    }

    #[test]
    fn test_rfc_3339_second_subfraction() {
        assert_eq!(rfc_3339_second_subfraction(".01"), Ok(("", "01")));
        assert_eq!(rfc_3339_second_subfraction(".01-"), Ok(("-", "01")));
        assert_eq!(rfc_3339_second_subfraction(".01-01"), Ok(("-01", "01")));
        assert_eq!(rfc_3339_second_subfraction(".0101"), Ok(("", "0101")));
        assert_eq!(rfc_3339_second_subfraction(".010100"), Ok(("", "0101")));
        assert_eq!(rfc_3339_second_subfraction("."), Ok(("", "")));

        assert_errors(rfc_3339_second_subfraction("0"));
        assert_errors(rfc_3339_second_subfraction("-01"));
        assert_errors(rfc_3339_second_subfraction(" .01"));
    }

    #[test]
    fn test_rfc_3339_time_offset() {
        assert_eq!(rfc_3339_time_offset("z"), Ok(("", RFC3339TimeOffset::Z)));
        assert_eq!(rfc_3339_time_offset("Z"), Ok(("", RFC3339TimeOffset::Z)));
        assert_eq!(
            rfc_3339_time_offset("+00:00"),
            Ok(("", RFC3339TimeOffset::NumberOffset("00", "00", true)))
        );
        assert_eq!(
            rfc_3339_time_offset("-01:23"),
            Ok(("", RFC3339TimeOffset::NumberOffset("01", "23", false)))
        );

        assert_errors(rfc_3339_time_offset("+1:23"));
        assert_errors(rfc_3339_time_offset("1:23"));
        assert_errors(rfc_3339_time_offset("1:1"));
        assert_errors(rfc_3339_time_offset("1:01"));
        assert_errors(rfc_3339_time_offset("01:1"));
        assert_errors(rfc_3339_time_offset("+-01:01"));
        assert_errors(rfc_3339_time_offset("-+01:01"));
    }

    #[test]
    fn test_rfc_3339_partial_time() {
        assert_eq!(
            rfc_3339_partial_time("01:01:01"),
            Ok((
                "",
                RFC3339PartialTime {
                    hour: "01",
                    minute: "01",
                    second: "01",
                    subfraction: None,
                }
            ))
        );
        assert_eq!(
            rfc_3339_partial_time("01:01:01.01"),
            Ok((
                "",
                RFC3339PartialTime {
                    hour: "01",
                    minute: "01",
                    second: "01",
                    subfraction: Some("01"),
                }
            ))
        );
    }

    #[test]
    fn test_rfc_3339_time() {
        assert_eq!(
            rfc_3339_time("01:01:01Z"),
            Ok((
                "",
                RFC3339Time {
                    partial: RFC3339PartialTime {
                        hour: "01",
                        minute: "01",
                        second: "01",
                        subfraction: None,
                    },
                    offset: RFC3339TimeOffset::Z,
                }
            ))
        );
        assert_eq!(
            rfc_3339_time("01:01:01.01Z"),
            Ok((
                "",
                RFC3339Time {
                    partial: RFC3339PartialTime {
                        hour: "01",
                        minute: "01",
                        second: "01",
                        subfraction: Some("01"),
                    },
                    offset: RFC3339TimeOffset::Z,
                }
            ))
        );
        assert_eq!(
            rfc_3339_time("01:01:01+00:00"),
            Ok((
                "",
                RFC3339Time {
                    partial: RFC3339PartialTime {
                        hour: "01",
                        minute: "01",
                        second: "01",
                        subfraction: None,
                    },
                    offset: RFC3339TimeOffset::NumberOffset("00", "00", true),
                }
            ))
        );
        assert_eq!(
            rfc_3339_time("01:01:01-01:30"),
            Ok((
                "",
                RFC3339Time {
                    partial: RFC3339PartialTime {
                        hour: "01",
                        minute: "01",
                        second: "01",
                        subfraction: None,
                    },
                    offset: RFC3339TimeOffset::NumberOffset("01", "30", false),
                }
            ))
        );
    }

    #[test]
    fn test_rfc_3339_date() {
        assert_eq!(
            rfc_3339_date("2000-01-01"),
            Ok((
                "",
                RFC3339Date {
                    year: "2000",
                    month: "01",
                    day: "01",
                }
            ))
        );
    }

    #[test]
    fn test_rfc_3339_date_time() {
        assert_eq!(
            rfc_3339_date_time("2000-01-01T01:01:01Z"),
            Ok((
                "",
                RFC3339DateTime {
                    date: RFC3339Date {
                        year: "2000",
                        month: "01",
                        day: "01",
                    },
                    time: RFC3339Time {
                        partial: RFC3339PartialTime {
                            hour: "01",
                            minute: "01",
                            second: "01",
                            subfraction: None,
                        },
                        offset: RFC3339TimeOffset::Z,
                    },
                }
            ))
        );
        assert_eq!(
            rfc_3339_date_time("2000-01-01 01:01:01.01-05:15"),
            Ok((
                "",
                RFC3339DateTime {
                    date: RFC3339Date {
                        year: "2000",
                        month: "01",
                        day: "01",
                    },
                    time: RFC3339Time {
                        partial: RFC3339PartialTime {
                            hour: "01",
                            minute: "01",
                            second: "01",
                            subfraction: Some("01"),
                        },
                        offset: RFC3339TimeOffset::NumberOffset("05", "15", false),
                    },
                }
            ))
        );
    }
}
