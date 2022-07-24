#[derive(PartialEq, Debug, Clone)]
pub enum IntegerKind {
    Decimal,
    Hexadecimal,
    Octal,
    Binary,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Integer<'a> {
    pub kind: IntegerKind,
    pub value: &'a str,
    pub is_negative: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Float<'a> {
    pub integer: &'a str,
    pub is_negative: bool,
    pub fraction: Option<&'a str>,
    pub exponent: Option<&'a str>,
    pub exponent_is_negative: bool,
}
