use crate::helpers::sign;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while_m_n},
    character::complete::digit0,
    combinator::opt,
    sequence::{preceded, tuple},
};
use prettify::{concat, string, PrettifyDoc};

pub fn is_digit(chr: char) -> bool {
    ('0'..='9').contains(&chr)
}

pub fn rfc_3339_full_year(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, result) = take_while_m_n(4, 4, is_digit)(input)?;
    Ok((remainder, string(result)))
}

pub fn rfc_3339_month(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, result) = take_while_m_n(2, 2, is_digit)(input)?;
    Ok((remainder, string(result)))
}

pub fn rfc_3339_day(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, result) = take_while_m_n(2, 2, is_digit)(input)?;
    Ok((remainder, string(result)))
}

pub fn rfc_3339_hour(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, result) = take_while_m_n(2, 2, is_digit)(input)?;
    Ok((remainder, string(result)))
}

pub fn rfc_3339_minute(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, result) = take_while_m_n(2, 2, is_digit)(input)?;
    Ok((remainder, string(result)))
}

pub fn rfc_3339_second(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, result) = take_while_m_n(2, 2, is_digit)(input)?;
    Ok((remainder, string(result)))
}

pub fn rfc_3339_second_subfraction(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, mut result) = preceded(tag("."), digit0)(input)?;
    result = result.trim_end_matches('0');
    if result.is_empty() {
        Ok((remainder, string("")))
    } else {
        Ok((
            remainder,
            concat(vec![string("."), string(result.trim_end_matches('0'))]),
        ))
    }
}

fn time_offset_z(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, _) = tag_no_case("z")(input)?;
    Ok((remainder, string("Z")))
}

fn time_offset_number_offset(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, (sign, hour, _, minute)) =
        tuple((sign, rfc_3339_hour, tag(":"), rfc_3339_minute))(input)?;
    Ok((remainder, concat(vec![sign, hour, string(":"), minute])))
}

pub fn rfc_3339_time_offset(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, result) = alt((time_offset_z, time_offset_number_offset))(input)?;
    Ok((remainder, result))
}

pub fn rfc_3339_partial_time(input: &str) -> nom::IResult<&str, PrettifyDoc> {
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
        concat(vec![
            hour,
            string(":"),
            minute,
            string(":"),
            second,
            subfraction.unwrap_or_else(|| string("")),
        ]),
    ))
}

pub fn rfc_3339_time(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, (partial, offset)) =
        tuple((rfc_3339_partial_time, rfc_3339_time_offset))(input)?;
    Ok((remainder, concat(vec![partial, offset])))
}

pub fn rfc_3339_date(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, (year, _, month, _, day)) = tuple((
        rfc_3339_full_year,
        tag("-"),
        rfc_3339_month,
        tag("-"),
        rfc_3339_day,
    ))(input)?;
    Ok((
        remainder,
        concat(vec![year, string("-"), month, string("-"), day]),
    ))
}

pub fn rfc_3339_date_time(input: &str) -> nom::IResult<&str, PrettifyDoc> {
    let (remainder, (date, _, time)) = tuple((
        rfc_3339_date,
        alt((tag_no_case("T"), tag(" "))),
        rfc_3339_time,
    ))(input)?;
    Ok((remainder, concat(vec![date, string("T"), time])))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_errors, assert_formatted};

    #[test]
    fn test_rfc_3339_full_year() {
        assert_formatted(rfc_3339_full_year("2020"), ("", "2020"));
        assert_formatted(rfc_3339_full_year("2020-"), ("-", "2020"));
        assert_formatted(rfc_3339_full_year("2020-01"), ("-01", "2020"));
        assert_formatted(rfc_3339_full_year("20200"), ("0", "2020"));

        assert_errors(rfc_3339_full_year("203"));
        assert_errors(rfc_3339_full_year("-203"));
    }

    #[test]
    fn test_rfc_3339_month() {
        assert_formatted(rfc_3339_month("01"), ("", "01"));
        assert_formatted(rfc_3339_month("01-"), ("-", "01"));
        assert_formatted(rfc_3339_month("01-01"), ("-01", "01"));
        assert_formatted(rfc_3339_month("0101"), ("01", "01"));

        assert_errors(rfc_3339_month("0"));
        assert_errors(rfc_3339_month("-01"));
    }

    #[test]
    fn test_rfc_3339_day() {
        assert_formatted(rfc_3339_day("01"), ("", "01"));
        assert_formatted(rfc_3339_day("01-"), ("-", "01"));
        assert_formatted(rfc_3339_day("01-01"), ("-01", "01"));
        assert_formatted(rfc_3339_day("0101"), ("01", "01"));

        assert_errors(rfc_3339_day("0"));
        assert_errors(rfc_3339_day("-01"));
    }

    #[test]
    fn test_rfc_3339_hour() {
        assert_formatted(rfc_3339_hour("01"), ("", "01"));
        assert_formatted(rfc_3339_hour("01-"), ("-", "01"));
        assert_formatted(rfc_3339_hour("01-01"), ("-01", "01"));
        assert_formatted(rfc_3339_hour("0101"), ("01", "01"));

        assert_errors(rfc_3339_hour("0"));
        assert_errors(rfc_3339_hour("-01"));
    }

    #[test]
    fn test_rfc_3339_minute() {
        assert_formatted(rfc_3339_minute("01"), ("", "01"));
        assert_formatted(rfc_3339_minute("01-"), ("-", "01"));
        assert_formatted(rfc_3339_minute("01-01"), ("-01", "01"));
        assert_formatted(rfc_3339_minute("0101"), ("01", "01"));

        assert_errors(rfc_3339_minute("0"));
        assert_errors(rfc_3339_minute("-01"));
    }

    #[test]
    fn test_rfc_3339_second() {
        assert_formatted(rfc_3339_second("01"), ("", "01"));
        assert_formatted(rfc_3339_second("01-"), ("-", "01"));
        assert_formatted(rfc_3339_second("01-01"), ("-01", "01"));
        assert_formatted(rfc_3339_second("0101"), ("01", "01"));

        assert_errors(rfc_3339_second("0"));
        assert_errors(rfc_3339_second("-01"));
    }

    #[test]
    fn test_rfc_3339_second_subfraction() {
        assert_formatted(rfc_3339_second_subfraction(".01"), ("", ".01"));
        assert_formatted(rfc_3339_second_subfraction(".01-"), ("-", ".01"));
        assert_formatted(rfc_3339_second_subfraction(".01-01"), ("-01", ".01"));
        assert_formatted(rfc_3339_second_subfraction(".0101"), ("", ".0101"));
        assert_formatted(rfc_3339_second_subfraction(".010100"), ("", ".0101"));
        assert_formatted(rfc_3339_second_subfraction("."), ("", ""));

        assert_errors(rfc_3339_second_subfraction("0"));
        assert_errors(rfc_3339_second_subfraction("-01"));
        assert_errors(rfc_3339_second_subfraction(" .01"));
    }

    #[test]
    fn test_rfc_3339_time_offset() {
        assert_formatted(rfc_3339_time_offset("z"), ("", "Z"));
        assert_formatted(rfc_3339_time_offset("Z"), ("", "Z"));
        assert_formatted(rfc_3339_time_offset("+00:00"), ("", "+00:00"));
        assert_formatted(rfc_3339_time_offset("-01:23"), ("", "-01:23"));

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
        assert_formatted(rfc_3339_partial_time("01:01:01"), ("", "01:01:01"));
        assert_formatted(rfc_3339_partial_time("01:01:01.01"), ("", "01:01:01.01"));
    }

    #[test]
    fn test_rfc_3339_time() {
        assert_formatted(rfc_3339_time("01:01:01Z"), ("", "01:01:01Z"));
        assert_formatted(rfc_3339_time("01:01:01.01Z"), ("", "01:01:01.01Z"));
        assert_formatted(rfc_3339_time("01:01:01+00:00"), ("", "01:01:01+00:00"));
        assert_formatted(rfc_3339_time("01:01:01-01:30"), ("", "01:01:01-01:30"));
    }

    #[test]
    fn test_rfc_3339_date() {
        assert_formatted(rfc_3339_date("2000-01-01"), ("", "2000-01-01"));
    }

    #[test]
    fn test_rfc_3339_date_time() {
        assert_formatted(
            rfc_3339_date_time("2000-01-01T01:01:01Z"),
            ("", "2000-01-01T01:01:01Z"),
        );
        assert_formatted(
            rfc_3339_date_time("2000-01-01 01:01:01.01-05:15"),
            ("", "2000-01-01T01:01:01.01-05:15"),
        );
    }
}
