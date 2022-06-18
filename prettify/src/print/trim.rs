use regex::Regex;

pub fn trim(out: &mut Vec<String>) -> usize {
    if out.is_empty() {
        return 0;
    }

    let all_whitespace = Regex::new(r"^[\t ]*$").unwrap();
    let mut trim_count = 0;
    while !out.is_empty() && all_whitespace.is_match(&out[out.len() - 1]) {
        trim_count += out.pop().unwrap().len();
    }

    let trailing_whitespace = Regex::new(r"([\t ]*)$").unwrap();
    if !out.is_empty() && trailing_whitespace.is_match(&out[out.len() - 1]) {
        let captures = trailing_whitespace.captures(&out[out.len() - 1]).unwrap();
        trim_count += &captures[0].len();
        out.pop();
        out.push(
            trailing_whitespace
                .replace(&out[out.len() - 1], "")
                .to_string(),
        );
    }

    trim_count
}
