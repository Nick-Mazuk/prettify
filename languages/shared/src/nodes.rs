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

#[derive(PartialEq, Debug, Clone)]
pub enum RFC3339TimeOffset<'a> {
    Z,
    //     hour,    minute,  is_negative
    NumberOffset(&'a str, &'a str, bool),
}

#[derive(PartialEq, Debug, Clone)]
pub struct RFC3339PartialTime<'a> {
    pub hour: &'a str,
    pub minute: &'a str,
    pub second: &'a str,
    pub subfraction: Option<&'a str>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct RFC3339Time<'a> {
    pub partial: RFC3339PartialTime<'a>,
    pub offset: RFC3339TimeOffset<'a>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct RFC3339Date<'a> {
    pub year: &'a str,
    pub month: &'a str,
    pub day: &'a str,
}

#[derive(PartialEq, Debug, Clone)]
pub struct RFC3339DateTime<'a> {
    pub date: RFC3339Date<'a>,
    pub time: RFC3339Time<'a>,
}
