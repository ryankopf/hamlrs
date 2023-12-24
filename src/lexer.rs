use crate::token::Token;

enum SpecialChar {
    Space,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    PercentageSign,
    Period,
    Equal,
    DoubleQuote,
    SingleQuote,
    BackSlash,
    ForwardSlash,
    Hashtag,
    LessThan,
    GreaterThan,
    Exclamation,
    Ampersand,
    Tilde,
    Newline,
}

trait TokenAction {
    fn from_char(ch: char) -> Option<Self> where Self: Sized;
    fn to_token(&self) -> Token;
    fn is_special(ch: char) -> bool;
}

impl TokenAction for SpecialChar {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            ' ' => Some(SpecialChar::Space),
            '(' => Some(SpecialChar::OpenParen),
            ')' => Some(SpecialChar::CloseParen),
            '{' => Some(SpecialChar::OpenBrace),
            '}' => Some(SpecialChar::CloseBrace),
            '%' => Some(SpecialChar::PercentageSign),
            '.' => Some(SpecialChar::Period),
            '=' => Some(SpecialChar::Equal),
            '\"' => Some(SpecialChar::DoubleQuote),
            '\'' => Some(SpecialChar::SingleQuote),
            '\\' => Some(SpecialChar::BackSlash),
            '/' => Some(SpecialChar::ForwardSlash),
            '#' => Some(SpecialChar::Hashtag),
            '<' => Some(SpecialChar::LessThan),
            '>' => Some(SpecialChar::GreaterThan),
            '!' => Some(SpecialChar::Exclamation),
            '&' => Some(SpecialChar::Ampersand),
            '~' => Some(SpecialChar::Tilde),
            '\n' => Some(SpecialChar::Newline),
            _ => None,
        }
    }

    fn to_token(&self) -> Token {
        match self {
            SpecialChar::Space => Token::Whitespace(),
            SpecialChar::OpenParen => Token::OpenParen(),
            SpecialChar::CloseParen => Token::CloseParen(),
            SpecialChar::OpenBrace => Token::OpenBrace(),
            SpecialChar::CloseBrace => Token::CloseBrace(),
            SpecialChar::PercentageSign => Token::PercentageSign(),
            SpecialChar::Period => Token::Period(),
            SpecialChar::Equal => Token::Equal(),
            SpecialChar::DoubleQuote => Token::DoubleQuote(),
            SpecialChar::SingleQuote => Token::SingleQuote(),
            SpecialChar::BackSlash => Token::BackSlash(),
            SpecialChar::ForwardSlash => Token::ForwardSlash(),
            SpecialChar::Hashtag => Token::Hashtag(),
            SpecialChar::LessThan => Token::LessThan(),
            SpecialChar::GreaterThan => Token::GreaterThan(),
            SpecialChar::Exclamation => Token::Exclamation(),
            SpecialChar::Ampersand => Token::Ampersand(),
            SpecialChar::Tilde => Token::Tilde(),
            SpecialChar::Newline => Token::Newline(),
        }
    }

    fn is_special(ch: char) -> bool {
        Self::from_char(ch).is_some()
    }
}


enum LexerState {
    Default,
    SingleLineComment,
    MultiLineComment,
    // Add other states as needed
}

pub fn lex(haml: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut cursor = 0;
    let chars: Vec<char> = haml.chars().collect();
    let mut state = LexerState::Default;

    while cursor < chars.len() {
        let ch = chars[cursor];

        match state {
            LexerState::Default => {
                if SpecialChar::is_special(ch) {
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }

                    if ch == '/' && chars.get(cursor + 1) == Some(&'/') {
                        state = LexerState::SingleLineComment;
                        cursor += 1; // Skip the next '/' character
                    } else if ch == '/' && chars.get(cursor + 1) == Some(&'*') {
                        state = LexerState::MultiLineComment;
                        cursor += 1; // Skip the next '*' character
                    } else if let Some(special) = SpecialChar::from_char(ch) {
                        tokens.push(special.to_token());
                    }
                } else {
                    buffer.push(ch);
                }
            },
            LexerState::SingleLineComment => {
                if ch == '\n' {
                    tokens.push(Token::Newline());
                    state = LexerState::Default;
                } // Else, ignore characters within single-line comment
            },
            LexerState::MultiLineComment => {
                if ch == '*' && chars.get(cursor + 1) == Some(&'/') {
                    state = LexerState::Default;
                    cursor += 1; // Skip the next '/' character
                } // Else, ignore characters within multiline comment
            },
            // Add handling for other states as needed
        }

        cursor += 1;
    }

    if !buffer.is_empty() {
        tokens.push(Token::Text(buffer));
    }

    tokens
}

