pub fn is_alphanumeric_or_underscore_or_dash(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}
