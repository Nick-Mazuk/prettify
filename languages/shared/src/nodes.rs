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
