use hamlrs::lexer::lex;
use hamlrs::token::Token;

#[test]
fn test_single_character_tokens() {
    let input = "{}()";
    let expected_tokens = vec![
        Token::OpenBrace(),
        Token::CloseBrace(),
        Token::OpenParen(),
        Token::CloseParen(),
    ];
    let actual_tokens = lex(input);
    assert_eq!(actual_tokens, expected_tokens);
}

#[test]
fn test_single_line_comments() {
    let input = "text // this is a comment\nnext line";
    let expected_tokens = vec![
        Token::Text("text".to_string()),
        Token::Whitespace(),
        Token::Newline(),
        Token::Text("next".to_string()),
        Token::Whitespace(),
        Token::Text("line".to_string()),
    ];
    let actual_tokens = lex(input);
    assert_eq!(actual_tokens, expected_tokens);
}


#[test]
fn test_multiline_comments() {
    let input = "text /* multiline\ncomment */next line";
    let expected_tokens = vec![
        Token::Text("text".to_string()),
        Token::Whitespace(),
        Token::Text("next".to_string()),
        Token::Whitespace(),
        Token::Text("line".to_string()),
    ];
    let actual_tokens = lex(input);
    assert_eq!(actual_tokens, expected_tokens);
}
