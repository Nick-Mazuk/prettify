pub fn assert_errors<T>(response: nom::IResult<&str, T>) {
    assert!(match response {
        Ok(_) => false,
        Err(_) => true,
    })
}
