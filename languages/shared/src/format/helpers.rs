pub fn add_integer_underscores(value: &str) -> String {
    if value.len() <= 3 || value.contains("_") {
        value.to_string()
    } else {
        let mut new_value = String::new();
        for (index, char) in value.chars().rev().enumerate() {
            if index > 0 && index % 3 == 0 {
                new_value.push('_');
            }
            new_value.push(char);
        }
        new_value.chars().rev().collect::<String>()
    }
}

pub fn add_integer_underscores_reverse(value: &str) -> String {
    if value.len() <= 3 || value.contains("_") {
        value.to_string()
    } else {
        let mut new_value = String::new();
        for (index, char) in value.chars().enumerate() {
            if index > 0 && index % 3 == 0 {
                new_value.push('_');
            }
            new_value.push(char);
        }
        new_value.chars().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_integer_underscores_test() {
        assert_eq!(add_integer_underscores("12345"), "12_345");
        assert_eq!(add_integer_underscores("1234567"), "1_234_567");
        assert_eq!(add_integer_underscores("12345678"), "12_345_678");
        assert_eq!(add_integer_underscores("123456789"), "123_456_789");
        assert_eq!(add_integer_underscores("1_2345"), "1_2345");
    }

    #[test]
    fn add_integer_underscores_reverse_test() {
        assert_eq!(add_integer_underscores_reverse("12345"), "123_45");
        assert_eq!(add_integer_underscores_reverse("1234567"), "123_456_7");
        assert_eq!(add_integer_underscores_reverse("12345678"), "123_456_78");
        assert_eq!(add_integer_underscores_reverse("123456789"), "123_456_789");
        assert_eq!(add_integer_underscores_reverse("1_2345"), "1_2345");
    }
}
