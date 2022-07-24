use prettify::{print, PrettifyDoc};

pub fn assert_errors<T>(response: nom::IResult<&str, T>) {
    assert!(match response {
        Ok(_) => false,
        Err(_) => true,
    })
}

pub fn assert_formatted<'a>(
    result: nom::IResult<&'a str, PrettifyDoc<'a>>,
    (expected_remainder, expected_formatted): (&'a str, &'a str),
) {
    let (actual_remainder, actual_formatted) = print_result(result);
    assert_eq!(actual_formatted, expected_formatted.to_string());
    assert_eq!(actual_remainder, expected_remainder);
}

pub fn print_result<'a>(result: nom::IResult<&'a str, PrettifyDoc<'a>>) -> (&'a str, String) {
    let (remainder, doc) = result.unwrap();
    (remainder, print(doc))
}
