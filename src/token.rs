
#[derive(Debug, PartialEq)]
pub enum Token {
    Whitespace(),
    Text(String),
    OpenParen(),
    CloseParen(),
    OpenBrace(),
    CloseBrace(),
    PercentageSign(),
    Period(),
    Equal(),
    SingleQuote(),
    DoubleQuote(),
    ForwardSlash(),
    BackSlash(),
    Hashtag(),
    LessThan(),
    GreaterThan(),
    Exclamation(),
    Ampersand(),
    Tilde(),
    Newline(),
}
